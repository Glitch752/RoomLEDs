use std::{cmp::min, net::{self, Ipv4Addr}, sync::{Arc, LazyLock}, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    }, response::IntoResponse, routing::{get, post}, Json, Router
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use shared::{ServerToClientMessage, StatusUpdateMessage};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};
use tokio::time;
use tower_http::services::{ServeDir, ServeFile};

use crate::{render::{effects::{self, AnyEffect, AnyTemporaryEffect}, frame::Pixel}, LightingState, FRAME_TIMES_STORED, TOTAL_PIXELS};

static WEB_SERVER_PORT: u16 = shared::constants::API_PORT;

// This is a temporary solution; I intend to replace this with a way to
// compose effects using the web interface.
struct EffectPreset {
    name: String,
    icon: String,
    effect: Box<dyn Fn() -> Box<AnyEffect> + Send + Sync>
}

struct TemporaryEffectPreset {
    name: String,
    effect: Box<dyn Fn() -> Box<AnyTemporaryEffect> + Send + Sync>
}

static EFFECTS: LazyLock<Vec<EffectPreset>> = LazyLock::new(|| vec![
    EffectPreset {
        name: "Websocket Input".to_string(),
        icon: "fas fa-plug".to_string(),
        effect: Box::new(|| effects::WebsocketInputEffect::new())
    },
    EffectPreset {
        name: "Rainbow stripes".to_string(),
        icon: "fas fa-rainbow".to_string(),
        effect: Box::new(|| effects::StripeEffect::new(TOTAL_PIXELS as f64 / 28., vec![
            (255, 0, 0),
            (255, 100, 0),
            (255, 255, 0),
            (0, 255, 0),
            (0, 0, 255),
            (143, 0, 255),
            (255, 255, 255),
        ], 84.0))
    },
    EffectPreset {
        name: "Music visualizer".to_string(),
        icon: "fas fa-music".to_string(),
        effect: Box::new(|| effects::RotateEffect::new(
            effects::MusicVisualizerEffect::new(shared::constants::MUSIC_VISUALIZER_PORT),
            -219
        ))
    },
    EffectPreset {
        name: "Flashing red".to_string(),
        icon: "fas fa-bolt".to_string(),
        effect: Box::new(|| effects::FlashingColorEffect::new(1., Pixel::new(255, 0, 0, 1.0)))
    },
    EffectPreset {
        name: "Solid white".to_string(),
        icon: "fas fa-sun".to_string(),
        effect: Box::new(|| effects::SolidColorEffect::new(Pixel::new(255, 255, 255, 1.0), 0, TOTAL_PIXELS))
    },
    EffectPreset {
        name: "Solid black".to_string(),
        icon: "fas fa-moon".to_string(),
        effect: Box::new(|| effects::SolidColorEffect::new(Pixel::new(0, 0, 0, 1.0), 0, TOTAL_PIXELS))
    },
]);

static TEMPORARY_EFFECTS: LazyLock<Vec<TemporaryEffectPreset>> = LazyLock::new(|| vec![
    TemporaryEffectPreset {
        name: "1 second red".to_string(),
        effect: Box::new(|| effects::DurationTemporaryEffect::new(
            1.,
            effects::SolidColorEffect::new(Pixel::new(255, 0, 0, 1.0), 0, TOTAL_PIXELS)
        ))
    }
]);

pub async fn serve(lighting_state: Arc<LightingState>) {
    let serve_dir = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));

    let api_router = Router::new()
        .route("/light_positions", get(light_positions_handler))
        .route("/run_effect", post(run_effect_handler));

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
    let pixel_locations = state.render_state.lock().info.pixel_locations.clone();
    let pixel_locations = pixel_locations.iter().map(|location| (location.x, location.y)).collect::<Vec<_>>();
    Json(pixel_locations)
}

async fn run_effect_handler(
    State(state): State<Arc<LightingState>>,
    Json(effect_name): Json<String>
) -> impl IntoResponse {
    let effect = TEMPORARY_EFFECTS
        .iter()
        .find(|preset| preset.name == effect_name)
        .map(|preset| (preset.effect)());
    
    if let Some(effect) = effect {
        state.render_state.lock().temporary_effect_compositor.add_effect(effect);
    }

    "OK"
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

    let light_positions = state.render_state.lock().info.pixel_locations.clone()
        .iter().map(|location| shared::LightPosition {
            x: location.x,
            y: location.y
        }).collect();

    let effect_presets = EFFECTS.iter().map(|preset| shared::EffectPreset {
        name: preset.name.clone(),
        icon: preset.icon.clone()
    }).collect();

    websocket_sender.send(ServerToClientMessage::Initialize(shared::InitializeMessage {
        light_positions,
        effect_presets
    }).into()).await.unwrap();

    // While this stream is open, periodically (20 times per second) send an update
    // to the client with the current state of the lights.
    if let Err(e) = send_frequent_state_update(&mut websocket_sender, state.clone()).await {
        eprintln!("Error sending state update: {:?}", e);
        return;
    }

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
                            handle_client_message(text, &state).await;
                        }
                        Message::Binary(data) => {
                            state.render_state.lock().info.websocket_input = Some(data);
                        }
                        Message::Close(_) => break,
                        _ => ()
                    }
                }
            }
            _ = fast_interval.tick() => {
                if let Err(e) = send_frequent_state_update(&mut websocket_sender, state.clone()).await {
                    eprintln!("Error sending state update: {:?}", e);
                    break;
                }
            }
            _ = slow_interval.tick() => {
                send_infrequent_state_update(&mut websocket_sender, &mut system).await;
            }
        }
    }
}

// Attempts to deserialize into a ClientToServerMessage and handle it
async fn handle_client_message(message: String, state: &Arc<LightingState>) {
    let deserialized_message: Result<shared::ClientToServerMessage, _> = serde_json::from_str(&message);
    if let Ok(message) = deserialized_message {
        match message {
            shared::ClientToServerMessage::UsePreset(preset_message) => {
                let effect = EFFECTS
                    .iter()
                    .find(|preset| preset.name == preset_message.preset_name)
                    .map(|preset| (preset.effect)());
                if let Some(effect) = effect {
                    state.render_state.lock().effect = effect;
                }
            }
        }
    } else {
        println!("Received invalid message: {}", message);
    }
}

async fn send_frequent_state_update(sender: &mut WebsocketSender, state: Arc<LightingState>) -> Result<(), axum::Error> {
    let (message, pixel_data) = {
        let render_info = &state.render_state.lock().info;

        let frames_to_average =  min(FRAME_TIMES_STORED, render_info.frames);
        let frame_times = render_info.frame_times.iter().take(frames_to_average).cloned();

        let message = ServerToClientMessage::StatusUpdate(StatusUpdateMessage {
            frames: render_info.frames as u32,
            average_window: frames_to_average as u32,
            average_frame_time: frame_times.clone().sum::<f64>() / frames_to_average as f64,
            max_frame_time: frame_times.clone().fold(0.0, f64::max),
            min_frame_time: frame_times.clone().fold(f64::INFINITY, f64::min),
            debug_text: render_info.debug_text.clone()
        });

        (message, render_info.current_presented_frame.as_ref().unwrap().pixel_data)
    };

    sender.send(message).await?;

    // We also send a binary message with the current pixel data
    sender.send_binary(pixel_data.to_vec()).await?;

    Ok(())
}

async fn send_infrequent_state_update(sender: &mut WebsocketSender, system: &mut sysinfo::System) {    
    system.refresh_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    );

    let message = ServerToClientMessage::SystemStatusUpdate(shared::SystemStatusUpdateMessage {
        global_cpu: system.global_cpu_usage(),
        available_memory: system.available_memory() as f64,
        total_memory: system.total_memory() as f64,
        used_swap: system.used_swap() as f64
    });
    sender.send(message).await.unwrap();
}