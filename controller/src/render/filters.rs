use super::frame::PresentedFrame;

mod gamma_correction;

pub use gamma_correction::GammaCorrectionFilter;

// TODO: Make some filters apply after storing the frame to send to the web interface
// This allows us to avoid applying gamma correction before sending the frame to the web interface, 
// since it looks pretty bad when applied twice

/// A filter is a render construct that modifies a frame of pixel data.
/// They are used for final post-processing after the entire frame has been rendered.
pub trait Filter {
    fn apply(&self, frame: PresentedFrame) -> PresentedFrame;
}