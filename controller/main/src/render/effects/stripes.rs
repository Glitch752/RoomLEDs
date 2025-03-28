use std::time::Duration;

use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::{render::frame::{Frame, PixelColor}, RenderInfo, TOTAL_PIXELS};

use super::{AnyEffect, Effect};

/// The stripes effect renders a rotating pattern with stripes of color.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct StripeEffect {
    stripe_width: f64,
    stripe_colors: Vec<PixelColor>,
    speed: f64,
}

impl StripeEffect {
    /// Creates a new stripes effect with the specified stripe width, colors, and speed.
    #[allow(unused)]
    pub fn new(stripe_width: f64, stripe_colors: Vec<PixelColor>, speed: f64) -> AnyEffect {
        StripeEffect {
            stripe_width,
            stripe_colors,
            speed,
        }.into()
    }
}

impl Effect for StripeEffect {
    fn render(&mut self, _delta: Duration, render_info: &mut RenderInfo) -> Frame {
        let mut frame = Frame::empty();

        for i in 0..TOTAL_PIXELS {
            let stripe_pos = (i as f64 + render_info.time * self.speed).round();

            let stripe_index = (stripe_pos / self.stripe_width).floor() as usize % self.stripe_colors.len();
            let stripe_color = &self.stripe_colors[stripe_index];
            let rgb = color_space::Rgb::new(
                stripe_color.r as f64,
                stripe_color.g as f64,
                stripe_color.b as f64,
            );

            let fade = 1. - (stripe_pos % self.stripe_width) / self.stripe_width;

            let mut pixel: PixelColor = rgb.into();
            pixel.alpha = fade * stripe_color.alpha;

            frame.set_pixel(i, pixel);
        }

        frame
    }
}