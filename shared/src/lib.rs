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
    pub frames: u32,
    pub average_window: u32,
    pub average_frame_time: f64,
    pub max_frame_time: f64,
    pub min_frame_time: f64,
    pub debug_text: String,
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