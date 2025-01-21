use std::{cmp::min, net::{self, Ipv4Addr}, sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    }, response::IntoResponse, routing::get, Json, Router
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use shared::{ServerToClientMessage, StatusUpdateMessage};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};
use tokio::time;
use tower_http::services::{ServeDir, ServeFile};

use crate::{LightingState, FRAME_TIMES_STORED};

static WEB_SERVER_PORT: u16 = 3000;

pub async fn serve(lighting_state: Arc<LightingState>) {
    let serve_dir = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));

    let api_router = Router::new()
        .route("/light_positions", get(light_positions_handler));

    let app = Router::new()
        .route("/websocket", get(websocket_handler))
        .nest("/api", api_router)
        .fallback_service(serve_dir)
        .with_state(lighting_state);

    let listener = tokio::net::TcpListener::bind(
            net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, WEB_SERVER_PORT))
        )
        .await
        .unwrap();

    // TODO: Switch to a proper logger like tracing

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn light_positions_handler(State(state): State<Arc<LightingState>>) -> impl IntoResponse {
    let pixel_locations = state.render_state.lock().pixel_locations.clone();
    let pixel_locations = pixel_locations.iter().map(|location| (location.x, location.y)).collect::<Vec<_>>();
    Json(pixel_locations)
}

async fn websocket_handler(
  ws: WebSocketUpgrade,
  State(state): State<Arc<LightingState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

struct WebsocketSender {
    sender: SplitSink<WebSocket, Message>
}

impl WebsocketSender {
    async fn send(&mut self, message: ServerToClientMessage) -> Result<(), axum::Error> {
        self.sender.send(Message::Text(serde_json::to_string(&message).unwrap())).await
    }
    async fn send_binary(&mut self, message: Vec<u8>) -> Result<(), axum::Error> {
        self.sender.send(Message::Binary(message)).await
    }
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<LightingState>) {
    // By splitting, we can send and receive at the same time.
    let (sender, mut receiver) = stream.split();
    let mut websocket_sender = WebsocketSender { sender };

    let light_positions = state.render_state.lock().pixel_locations.clone()
        .iter().map(|location| shared::LightPosition {
            x: location.x,
            y: location.y
        }).collect();

    websocket_sender.send(ServerToClientMessage::Initialize(shared::InitializeMessage {
        light_positions
    }).into()).await.unwrap();

    // While this stream is open, periodically (20 times per second) send an update
    // to the client with the current state of the lights.
    send_frequent_state_update(&mut websocket_sender, state.clone()).await;

    let mut system = sysinfo::System::new();

    let mut fast_interval = time::interval(Duration::from_secs(1) / 20);
    let mut slow_interval = time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            Some(message) = receiver.next() => {
                // We ignore any messages from the client for now
                if let Ok(message) = message {
                    match message {
                        Message::Text(text) => {
                            println!("Received message: {}", text);
                        }
                        Message::Close(_) => break,
                        _ => ()
                    }
                }
            }
            _ = fast_interval.tick() => {
                send_frequent_state_update(&mut websocket_sender, state.clone()).await;
            }
            _ = slow_interval.tick() => {
                send_infrequent_state_update(&mut websocket_sender, &mut system).await;
            }
        }
    }
}

async fn send_frequent_state_update(sender: &mut WebsocketSender, state: Arc<LightingState>) {
    let (message, pixel_data) = {
        let render_state = state.render_state.lock();

        let frames_to_average =  min(FRAME_TIMES_STORED, render_state.frames);
        let frame_times = render_state.frame_times.iter().take(frames_to_average).cloned();

        let message = ServerToClientMessage::StatusUpdate(StatusUpdateMessage {
            frames: render_state.frames as u32,
            average_window: frames_to_average as u32,
            average_frame_time: frame_times.clone().sum::<f64>() / frames_to_average as f64,
            max_frame_time: frame_times.clone().fold(0.0, f64::max),
            min_frame_time: frame_times.clone().fold(f64::INFINITY, f64::min),
            debug_text: render_state.debug_text.clone()
        });

        (message, render_state.current_presented_frame.as_ref().unwrap().pixel_data)
    };

    sender.send(message).await.unwrap();

    // We also send a binary message with the current pixel data
    sender.send_binary(pixel_data.to_vec()).await.unwrap();
}

async fn send_infrequent_state_update(sender: &mut WebsocketSender, system: &mut sysinfo::System) {    
    system.refresh_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    );

    let message = ServerToClientMessage::SystemStatusUpdate(shared::SystemStatusUpdateMessage {
        global_cpu: system.global_cpu_usage(),
        free_memory: system.free_memory() as f64,
        total_memory: system.total_memory() as f64,
        used_swap: system.used_swap() as f64
    });
    sender.send(message).await.unwrap();
}