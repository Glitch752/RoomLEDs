use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod constants;

pub type LightPositions = Vec<LightPosition>;

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "index.ts")]
pub struct LightPosition {
    pub x: f32,
    pub y: f32
}

#[derive(TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export, export_to = "index.ts")]
pub enum ServerToClientMessage {
    StatusUpdate(StatusUpdateMessage),
    SystemStatusUpdate(SystemStatusUpdateMessage),
    Initialize(InitializeMessage),
}

#[derive(TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export, export_to = "index.ts")]
pub enum ClientToServerMessage {
    // Pixel data updates use a binary message instead of JSON
    // PixelDataUpdate(Vec<u8>),
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "index.ts")]
pub struct EffectPresetList {
    pub effects: Vec<EffectPreset>
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "index.ts")]
pub struct EffectPreset {
    pub name: String,
    pub icon: String
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "index.ts")]
pub struct TemporaryEffectList {
    pub effects: Vec<String>
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "index.ts")]
pub struct InitializeMessage {
    pub light_positions: LightPositions,
    pub effect_presets: Vec<EffectPreset>
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "index.ts")]
pub struct StatusUpdateMessage {
    /// The total number of frames rendered
    pub frames: u32,
    /// The number of frames currently used for frmae time statistics
    pub average_window: u32,
    /// The average time it took to render a frame in milliseconds
    pub average_frame_time: f64,
    /// The maximum time it took to render a frame in milliseconds
    pub max_frame_time: f64,
    /// The minimum time it took to render a frame in milliseconds
    pub min_frame_time: f64,
    /// Misellaneous debug text that can be used for anything
    pub debug_text: String,
    
    /// If the lights are currently idle
    pub idle: bool
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "index.ts")]
pub struct SystemStatusUpdateMessage {
    pub global_cpu: f32,
    pub available_memory: f64,
    pub total_memory: f64,
    pub used_swap: f64
}

#[derive(TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export, export_to = "index.ts")]
pub enum MusicVisualizerMessage {
    UpdateSpectrum(Vec<u8>)
}