use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type LightPositions = Vec<LightPosition>;

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
pub struct LightPosition {
    x: f32,
    y: f32
}

#[derive(TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export)]
pub enum ServerToClientMessage {
    UpdatePixelData(Vec<u8>),
    LightPositionsUpdate(LightPositions),
    Initialize(InitializeMessage),
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
pub struct InitializeMessage {
    pub light_positions: LightPositions,
}

#[derive(TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export)]
pub enum MusicVisualizerMessage {
    UpdateSpectrum(Vec<u8>)
}