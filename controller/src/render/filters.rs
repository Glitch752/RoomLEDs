use super::frame::PresentedFrame;

mod gamma_correction;

pub use gamma_correction::GammaCorrectionFilter;

/// A filter is a render construct that modifies a frame of pixel data.
/// They are used for post-processing effects.
pub trait Filter {
    fn apply(&self, frame: PresentedFrame) -> PresentedFrame;
}