use color_space::Rgb;

use crate::TOTAL_PIXELS;

/// A pixel is a single unit of color data with an alpha value.
#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: f64
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, alpha: f64) -> Pixel {
        Pixel {
            r,
            g,
            b,
            alpha
        }
    }
}

impl From<Rgb> for Pixel {
    fn from(rgb: Rgb) -> Self {
        Pixel {
            r: rgb.r as u8,
            g: rgb.g as u8,
            b: rgb.b as u8,
            alpha: 1.0
        }
    }
}

/// A frame is a single set of pixel data.
pub struct Frame {
    pub pixel_data: Vec<Pixel>
}

impl Frame {
    pub fn empty() -> Frame {
        Frame {
            pixel_data: vec![Pixel::new(0, 0, 0, 0.0); 812]
        }
    }
}


/// A presented frame is a frame that has been composited and is ready to be sent to the LEDs.
/// Post-processing filters are applied to presented frames, since they shouldn't care about alpha.
#[derive(Debug, Clone)]
pub struct PresentedFrame {
    pub pixel_data: [u8; TOTAL_PIXELS as usize * 3]
}

impl From<Frame> for PresentedFrame {
    fn from(frame: Frame) -> Self {
        let mut pixel_data = [0; TOTAL_PIXELS as usize * 3];

        for i in 0..TOTAL_PIXELS {
            let pixel = &frame.pixel_data[i as usize];
            let index = i as usize * 3;

            // We essentially layer the color on top of black when converting to a presented frame
            pixel_data[index] = (pixel.r as f64 * pixel.alpha) as u8;
            pixel_data[index + 1] = (pixel.g as f64 * pixel.alpha) as u8;
            pixel_data[index + 2] = (pixel.b as f64 * pixel.alpha) as u8;
        }

        PresentedFrame {
            pixel_data
        }
    }
}