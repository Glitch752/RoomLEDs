use crate::render::frame::PresentedFrame;

use super::Filter;

/// A gamma correction filter applies gamma correction to a frame.
/// Gamma correction is a nonlinear operation used to encode and decode luminance values to account for the nonlinear response of the human eye.
/// Essentially, it emphasizes darker colors.
/// The gamma correction value is typically around 2.2.
pub struct GammaCorrectionFilter {
    gamma: f64,
}

impl GammaCorrectionFilter {
    pub fn new(gamma: f64) -> GammaCorrectionFilter {
        GammaCorrectionFilter {
            gamma,
        }
    }
}

impl Filter for GammaCorrectionFilter {
    fn apply(&self, frame: PresentedFrame) -> PresentedFrame {
        let mut corrected_frame = PresentedFrame {
            pixel_data: [0; 812 * 3]
        };

        for i in 0..812 {
            let r = frame.pixel_data[i * 3] as f64 / 255.0;
            let g = frame.pixel_data[i * 3 + 1] as f64 / 255.0;
            let b = frame.pixel_data[i * 3 + 2] as f64 / 255.0;

            corrected_frame.pixel_data[i * 3] = (r.powf(self.gamma) * 255.0) as u8;
            corrected_frame.pixel_data[i * 3 + 1] = (g.powf(self.gamma) * 255.0) as u8;
            corrected_frame.pixel_data[i * 3 + 2] = (b.powf(self.gamma) * 255.0) as u8;
        }

        corrected_frame
    }
}