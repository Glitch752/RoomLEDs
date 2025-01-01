use super::frame::Frame;

mod additive;
mod alpha;

pub use additive::AdditiveCompositor;
pub use alpha::AlphaCompositor;

/// A compositor is a render construct that composites layers together.
/// Examples of compositors include additive blending, alpha blending, etc.
/// The compositor is responsible for blending the layers together to form a single frame.
pub trait Compositor {
    fn composite(&self, layers: Vec<Frame>) -> Frame;
}