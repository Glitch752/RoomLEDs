use crate::RenderState;
use super::frame::Frame;

mod stripes;

pub use stripes::StripeLayer;

/// A layer is a render construct that returns a frame of pixel data with opacity.
/// Layers are composited together to form the final frame.
pub trait Layer {
    fn render(&self, render_state: &RenderState) -> Frame;
}