use color_space::{Hsl, Rgb};
use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::TOTAL_PIXELS;

/// A pixel is a single unit of color data with an alpha value.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: f64
}

impl PixelColor {
    pub const BLACK: PixelColor = PixelColor {
        r: 0,
        g: 0,
        b: 0,
        alpha: 1.0
    };

    pub fn new(r: u8, g: u8, b: u8, alpha: f64) -> PixelColor {
        PixelColor {
            r,
            g,
            b,
            alpha
        }
    }

    pub fn with_alpha(&self, alpha: f64) -> PixelColor {
        PixelColor {
            r: self.r,
            g: self.g,
            b: self.b,
            alpha
        }
    }

    pub fn lerp(&self, other: &PixelColor, t: f64) -> PixelColor {
        PixelColor {
            r: (self.r as f64 * (1.0 - t) + other.r as f64 * t) as u8,
            g: (self.g as f64 * (1.0 - t) + other.g as f64 * t) as u8,
            b: (self.b as f64 * (1.0 - t) + other.b as f64 * t) as u8,
            alpha: self.alpha * (1.0 - t) + other.alpha * t
        }
    }
}

impl From<Rgb> for PixelColor {
    fn from(rgb: Rgb) -> Self {
        PixelColor {
            r: rgb.r as u8,
            g: rgb.g as u8,
            b: rgb.b as u8,
            alpha: 1.0
        }
    }
}

impl From<Hsl> for PixelColor {
    fn from(hsl: Hsl) -> Self {
        let rgb: Rgb = hsl.into();
        rgb.into()
    }
}

impl From<(u8, u8, u8)> for PixelColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        PixelColor {
            r,
            g,
            b,
            alpha: 1.0
        }
    }
}

/// A frame is a single set of pixel data.
pub struct Frame {
    pixel_data: Vec<PixelColor>
}

impl Frame {
    pub fn empty() -> Frame {
        Frame {
            pixel_data: vec![PixelColor::new(0, 0, 0, 0.0); 812]
        }
    }

    pub fn set_pixel(&mut self, index: u32, pixel: PixelColor) {
        self.pixel_data[index as usize] = pixel;
    }

    pub fn get_pixel(&self, index: u32) -> PixelColor {
        self.pixel_data[index as usize].clone()
    }

    pub fn get_pixel_mut(&mut self, index: u32) -> &mut PixelColor {
        &mut self.pixel_data[index as usize]
    }
}


/// A presented frame is a frame that has been composited and is ready to be sent to the LEDs.
/// Post-processing filters are applied to presented frames, since they shouldn't care about alpha.
#[derive(Debug, Clone)]
pub struct PresentedFrame {
    pub pixel_data: [u8; TOTAL_PIXELS as usize * 3]
}

impl PresentedFrame {
    pub fn get_pixel(&self, index: u32) -> (u8, u8, u8) {
        let index = index as usize * 3;
        (self.pixel_data[index], self.pixel_data[index + 1], self.pixel_data[index + 2])
    }
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