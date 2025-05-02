use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};

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
