use axum::response::IntoResponse;
use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use futures_util::{SinkExt, StreamExt};

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

async fn handle_socket(mut socket: WebSocket) {
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
