#![allow(unused)]

use std::time::Duration;

use crate::RenderState;
use super::frame::Frame;

mod additive_compositor;
mod alpha_compositor;

mod stripes;
mod music_visualizer;

mod flashing_color;

mod rotate;

pub use additive_compositor::AdditiveCompositorEffect;
pub use alpha_compositor::AlphaCompositorEffect;
use serde::{Deserialize, Serialize};
pub use stripes::StripeEffect;
pub use music_visualizer::MusicVisualizerEffect;
pub use rotate::RotateEffect;
pub use flashing_color::FlashingColorEffect;
use ts_rs::TS;

/// An effect is a render construct that returns a frame of pixel data with opacity.
/// Effects can take other effects as an input.
pub trait Effect {
    fn render(&mut self, delta: Duration, render_state: &mut RenderState) -> Frame;
}

#[derive(TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export)]
pub enum AnyEffect {
    AdditiveCompositor(AdditiveCompositorEffect),
    AlphaCompositor(AlphaCompositorEffect),
    Stripe(StripeEffect),
    #[serde(skip)]
    // TODO: Implement serialization for MusicVisualizerEffect
    MusicVisualizer(MusicVisualizerEffect),
    Rotate(RotateEffect),
    FlashingColor(FlashingColorEffect)
}


macro_rules! from_effect {
    ($effect:ty, $variant:ident) => {
        impl From<$effect> for AnyEffect {
            fn from(effect: $effect) -> Self {
                AnyEffect::$variant(effect)
            }
        }
    };
}

impl Effect for AnyEffect {
    fn render(&mut self, delta: Duration, render_state: &mut RenderState) -> Frame {
        match self {
            AnyEffect::AdditiveCompositor(effect) => effect.render(delta, render_state),
            AnyEffect::AlphaCompositor(effect) => effect.render(delta, render_state),
            AnyEffect::Stripe(effect) => effect.render(delta, render_state),
            AnyEffect::MusicVisualizer(effect) => effect.render(delta, render_state),
            AnyEffect::Rotate(effect) => effect.render(delta, render_state),
            AnyEffect::FlashingColor(effect) => effect.render(delta, render_state)
        }
    }
}

from_effect!(AdditiveCompositorEffect, AdditiveCompositor);
from_effect!(AlphaCompositorEffect, AlphaCompositor);
from_effect!(StripeEffect, Stripe);
from_effect!(MusicVisualizerEffect, MusicVisualizer);
from_effect!(RotateEffect, Rotate);
from_effect!(FlashingColorEffect, FlashingColor);