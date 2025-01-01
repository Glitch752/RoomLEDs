use crate::{render::frame::{Frame, Pixel}, RenderState, TOTAL_PIXELS};

use super::Layer;

/// The stripes layer renders a rotating pattern with stripes of color.
pub struct StripeLayer {
    stripe_width: f64,
    stripe_colors: Vec<(u8, u8, u8)>,
    speed: f64,
}

impl StripeLayer {
    pub fn new(stripe_width: f64, stripe_colors: Vec<(u8, u8, u8)>, speed: f64) -> StripeLayer {
        StripeLayer {
            stripe_width,
            stripe_colors,
            speed,
        }
    }
}

impl Layer for StripeLayer {
    fn render(&mut self, render_state: &RenderState) -> Frame {
        let mut frame = Frame::empty();

        for i in 0..TOTAL_PIXELS {
            let stripe_pos = (i as f64 + render_state.time * self.speed).round();

            let stripe_index = (stripe_pos / self.stripe_width).floor() as usize % self.stripe_colors.len();
            let rgb = color_space::Rgb::new(
                self.stripe_colors[stripe_index].0 as f64,
                self.stripe_colors[stripe_index].1 as f64,
                self.stripe_colors[stripe_index].2 as f64,
            );

            let fade = 1. - (stripe_pos % self.stripe_width) / self.stripe_width;

            let mut pixel: Pixel = rgb.into();
            pixel.alpha = fade;

            frame.set_pixel(i, pixel);
        }

        frame
    }
}