use std::time::Duration;

use crate::RenderState;
use super::frame::Frame;

mod stripes;
mod music_visualizer;

pub use stripes::StripeLayer;
pub use music_visualizer::MusicVisualizerLayer;

/// A layer is a render construct that returns a frame of pixel data with opacity.
/// Layers are composited together to form the final frame.
pub trait Layer {
    fn render(&mut self, delta: Duration, render_state: &RenderState) -> Frame;
}