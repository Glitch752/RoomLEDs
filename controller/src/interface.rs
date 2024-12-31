use std::{net::{self, Ipv4Addr}, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tower_http::services::{ServeDir, ServeFile};

use crate::LightingState;

pub async fn serve(lighting_state: Arc<LightingState>) {
    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"));

    let app = Router::new()
        .route("/websocket", get(websocket_handler))
        .fallback_service(serve_dir)
        .with_state(lighting_state);

    let listener = tokio::net::TcpListener::bind(
            net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000))
        )
        .await
        .unwrap();

    // TODO: Switch to a proper logger like tracing

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(
  ws: WebSocketUpgrade,
  State(state): State<Arc<LightingState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<LightingState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // While this stream is open, periodically (20 times per second) send an update
    // to the client with the current state of the lights.

    // Clone the render state so we minimize the time we hold the lock
    let render_state = state.render_state.lock().clone();
    let message = json!({
        "frames": render_state.frames,
        "average_frame_time": render_state.frame_times.iter().sum::<f64>() / render_state.frames as f64,
    });
    sender.send(Message::Text(message.to_string())).await.unwrap();

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(message) = message {
            println!("Received message: {}", message);
            sender.send(Message::Text(message)).await.unwrap();
        }
    }
}