#![allow(unused)]

use std::time::Duration;

use crate::RenderState;
use super::frame::Frame;

mod additive_compositor;
mod alpha_compositor;

mod stripes;
mod music_visualizer;

mod rotate;

pub use additive_compositor::AdditiveCompositorEffect;
pub use alpha_compositor::AlphaCompositorEffect;
pub use stripes::StripeEffect;
pub use music_visualizer::MusicVisualizerEffect;
pub use rotate::RotateEffect;

/// An effect is a render construct that returns a frame of pixel data with opacity.
/// Effects can take other effects as an input.
pub trait Effect {
    fn render(&mut self, delta: Duration, render_state: &RenderState) -> Frame;
}