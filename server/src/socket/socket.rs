use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use tokio_stream::StreamExt;

// 简单使用，不需要使用tokio-tungstenite

pub async fn handle_socket(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_ws_stream)
}

async fn handle_ws_stream(mut socket: WebSocket) {
    println!("New WebSocket connection established!");

    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(msg) = msg {
            println!("Received message: {}", msg);
            let response = format!("received message: {}", msg);
            let _ = socket.send(Message::Text(Utf8Bytes::from(response))).await;
        }
    }

    println!("Connection closed!");
}
