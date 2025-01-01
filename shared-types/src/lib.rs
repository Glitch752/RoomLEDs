pub enum ControllerMessage {
    UpdateMusicVisualizer(MusicVisualizerMessage)
}

pub enum MusicVisualizerMessage {
    UpdateSpectrum(Vec<f32>)
}