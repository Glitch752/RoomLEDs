#![allow(unused)]

use std::time::Duration;

use crate::{RenderInfo, RenderState};
use super::frame::Frame;

mod additive_compositor;
mod alpha_compositor;

mod stripes;
mod music_visualizer;

mod flashing_color;

mod rotate;

pub use additive_compositor::AdditiveCompositorEffect;
pub use alpha_compositor::AlphaCompositorEffect;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
pub use stripes::StripeEffect;
pub use music_visualizer::MusicVisualizerEffect;
pub use rotate::RotateEffect;
pub use flashing_color::FlashingColorEffect;
use ts_rs::TS;

/// An effect is a render construct that returns a frame of pixel data with opacity.
/// Effects can take other effects as an input.
#[enum_dispatch]
pub trait Effect {
    fn render(&mut self, delta: Duration, render_info: &mut RenderInfo) -> Frame;
}

// TODO: Maybe we could use [typetag](https://github.com/dtolnay/typetag) instead
// to avoid this enum? I'm not sure if ts-rs will be able to create bindings for it, though.

#[derive(TS, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[ts(export)]
#[enum_dispatch(Effect)]
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