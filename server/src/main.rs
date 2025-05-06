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

const TICK_RATE: f32 = 0.02;

#[derive(Debug)]
pub struct ServerState {
    pub input_tx: mpsc::UnboundedSender<Input>,
    pub snapshot_rx: broadcast::Receiver<Snapshot>,
}

pub type SharedServerState = Arc<ServerState>;

#[derive(Deserialize)]
struct ConnectParams {
    id: String,
    team: Team,
}

pub async fn handle_socket(
    socket: WebSocket,
    shared_server_state: SharedServerState,
    params: ConnectParams,
) {
    println!("Client ID: {}", params.id);

    let (ws_sender, ws_receiver) = socket.split();

    // TODO: Allow for multiple concurrent games | here should send message to game manager

    tokio::spawn(receive_game_snapshots(
        ws_sender,
        shared_server_state.snapshot_rx.resubscribe(),
    ));

    forward_player_inputs(params, ws_receiver, shared_server_state.input_tx.clone()).await;
}

async fn receive_game_snapshots(
    mut ws_sender: SplitSink<WebSocket, Message>,
    mut snapshot_rx: broadcast::Receiver<Snapshot>,
) {
    while let Ok(snapshot) = snapshot_rx.recv().await {
        let serialized_bytes = rmp_serde::to_vec_named(&snapshot).unwrap();
        let axum_bytes: axum::body::Bytes = serialized_bytes.into();

        if let Err(e) = ws_sender.send(Message::Binary(axum_bytes)).await {
            println!("Failed to send snapshot: {}", e);
            break; // Exit the loop if connection is closed
        }
    }
}

async fn forward_player_inputs(
    params: ConnectParams,
    mut ws_receiver: SplitStream<WebSocket>,
    input_tx: mpsc::UnboundedSender<Input>,
) {
    // Send initial player assigned message
    input_tx
        .send(Input::CreatePlayer {
            team: params.team,
            id: params.id.clone(),
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
                Input::RemovePlayer { id } => {
                    input_tx.send(Input::RemovePlayer { id }).unwrap();
                }
                Input::PlayerMelee { player_id } => {
                    input_tx.send(Input::PlayerMelee { player_id }).unwrap();
                }
            }
        } else {
            println!("Received non-binary message: {:?}", input);
        }
    }

    println!("Client disconnected");

    // Inform game to remove player
    input_tx
        .send(Input::RemovePlayer { id: params.id })
        .unwrap();
}

async fn run_game_loop(
    mut input_rx: mpsc::UnboundedReceiver<Input>,
    snapshot_tx: broadcast::Sender<Snapshot>,
) {
    let mut game = Game::new(); // <-- exclusive owner
    let mut tick = tokio::time::interval(Duration::from_secs_f32(TICK_RATE));
    let mut snapshot_count = 0;
    let start_time = std::time::Instant::now();

    loop {
        tokio::select! {
            _ = tick.tick() => {
                // Process game tick
                game.step(TICK_RATE);

                snapshot_count += 1;
                // let elapsed = start_time.elapsed().as_secs_f32();
                let snapshot = game.make_snapshot();
                // println!(
                //     "Sending snapshot #{} ({:.2}s) - Players: {:?}",
                //     snapshot_count, elapsed, snapshot.players
                // );
                let _ = snapshot_tx.send(snapshot); // lagging clients drop
            }
            result = input_rx.recv() => {
                if let Some(cmd) = result {
                    game.apply_input(cmd).ok(); // impossible to dead-lock
                }
            }
        }
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

    // Get port from environment variable or use 8080 as default
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // run our app with hyper, listening globally on the specified port
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://localhost:{} (ws on /ws)", port);
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
        async move { handle_socket(socket, server_state, params).await }
    })
}
