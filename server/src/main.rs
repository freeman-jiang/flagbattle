use std::sync::Arc;
use std::time::Duration;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{
    Router,
    extract::Query,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use game::{Game, Input, Snapshot, Team};
use serde::Deserialize;
use tokio::sync::{broadcast, mpsc};

const TICK_RATE: f32 = 1.0;

#[derive(Debug)]
pub struct ServerState {
    pub input_tx: mpsc::UnboundedSender<Input>,
    pub snapshot_rx: broadcast::Receiver<Snapshot>,
}

pub type SharedServerState = Arc<ServerState>;

#[derive(Deserialize)]
struct ConnectParams {
    id: String,
}

pub async fn handle_socket(
    socket: WebSocket,
    shared_server_state: SharedServerState,
    client_id: String,
) {
    println!("Client ID: {}", client_id);

    let (ws_sender, ws_receiver) = socket.split();

    // TODO: Allow for multiple concurrent games | here should send message to game manager

    tokio::spawn(receive_game_snapshots(
        ws_sender,
        shared_server_state.snapshot_rx.resubscribe(),
    ));

    forward_player_inputs(client_id, ws_receiver, shared_server_state.input_tx.clone()).await;
}

async fn receive_game_snapshots(
    mut ws_sender: SplitSink<WebSocket, Message>,
    mut snapshot_rx: broadcast::Receiver<Snapshot>,
) {
    while let Ok(snapshot) = snapshot_rx.recv().await {
        println!("Received snapshot");
        let serialized_bytes = rmp_serde::to_vec_named(&snapshot).unwrap();
        let axum_bytes: axum::body::Bytes = serialized_bytes.into();

        ws_sender.send(Message::Binary(axum_bytes)).await.unwrap();
    }
}

async fn forward_player_inputs(
    client_id: String,
    mut ws_receiver: SplitStream<WebSocket>,
    input_tx: mpsc::UnboundedSender<Input>,
) {
    // Send initial player assigned message
    input_tx
        .send(Input::CreatePlayer {
            team: Team::Red,
            id: client_id,
        })
        .unwrap();

    while let Some(Ok(input)) = ws_receiver.next().await {
        if let Message::Binary(bytes) = input {
            // Deserialize the bytes into an Input
            let input = match rmp_serde::from_slice::<Input>(&bytes) {
                Ok(input) => input,
                Err(e) => {
                    println!("Failed to deserialize input: {}", e);
                    continue;
                }
            };

            match input {
                Input::PlayerMove {
                    player_id,
                    velocity,
                } => {
                    input_tx
                        .send(Input::PlayerMove {
                            player_id,
                            velocity,
                        })
                        .unwrap();
                }
                Input::CreatePlayer { team, id } => {
                    panic!("Wait this shouldn't happen")
                }
            }
        } else {
            println!("Received non-binary message: {:?}", input);
        }
    }

    println!("Client disconnected");
}

async fn run_game_loop(
    mut input_rx: mpsc::UnboundedReceiver<Input>,
    snapshot_tx: broadcast::Sender<Snapshot>,
) {
    let mut game = Game::new(); // <-- exclusive owner
    let mut tick = tokio::time::interval(Duration::from_secs_f32(TICK_RATE));

    loop {
        tick.tick().await;

        // Drain all player commands that arrived since last frame
        while let Ok(cmd) = input_rx.try_recv() {
            game.apply_input(cmd).ok(); // impossible to dead-lock
        }

        game.step(TICK_RATE);

        println!("Sending snapshot");
        let _ = snapshot_tx.send(game.make_snapshot()); // lagging clients drop
    }
}

#[tokio::main]
async fn main() {
    let (snapshot_tx, snapshot_rx) = broadcast::channel(16);
    let (input_tx, input_rx) = mpsc::unbounded_channel();

    let game_loop_handle = tokio::spawn(run_game_loop(input_rx, snapshot_tx));

    // Allows speaking with the game
    let shared_server_state = Arc::new(ServerState {
        input_tx,
        snapshot_rx,
    });

    // build our application with a websocket route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ws", get(ws_handler))
        .with_state(shared_server_state);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on http://localhost:8080 (ws on /ws)");
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    State(server_state): State<SharedServerState>,
    Query(params): Query<ConnectParams>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    println!("Client connected with id: {}", params.id);

    ws.on_upgrade(move |socket| {
        let server_state = server_state.clone();
        let client_id = params.id.clone();
        async move { handle_socket(socket, server_state, client_id).await }
    })
}
