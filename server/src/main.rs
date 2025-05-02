use std::time::Duration;

use axum::response::IntoResponse;
use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use game::{Game, Input, Snapshot};
use tokio::sync::{broadcast, mpsc};

const TICK_RATE: f32 = 1.0;

pub async fn handle_socket(mut socket: WebSocket) {
    // Echo incoming text messages back to the client
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            println!("Received message: {}", text);
            if socket.send(Message::Text(text)).await.is_err() {
                break; // client disconnected
            }
        }
    }
}

async fn game_loop(
    mut inbox: mpsc::UnboundedReceiver<Input>,
    snap_tx: broadcast::Sender<Snapshot>,
) {
    let mut game = Game::new(); // <-- exclusive owner
    let mut tick = tokio::time::interval(Duration::from_secs_f32(TICK_RATE));

    loop {
        tick.tick().await;

        // Drain all player commands that arrived since last frame
        while let Ok(cmd) = inbox.try_recv() {
            game.apply_input(cmd).ok(); // impossible to dead-lock
        }

        game.step(TICK_RATE);

        let _ = snap_tx.send(game.make_snapshot()); // lagging clients drop
    }
}

#[tokio::main]
async fn main() {
    // build our application with a websocket route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ws", get(ws_handler));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on http://localhost:8080 (ws on /ws)");
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}
