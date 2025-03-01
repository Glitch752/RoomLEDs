#![allow(unused)]

use super::frame::PresentedFrame;

mod gamma_correction;

pub use gamma_correction::GammaCorrectionFilter;

/// A filter is a render construct that modifies a frame of pixel data.
/// They are used for final post-processing after the entire frame has been rendered.
pub trait Filter {
    fn apply(&self, frame: PresentedFrame) -> PresentedFrame;
}