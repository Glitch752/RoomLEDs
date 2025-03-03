use std::time::Duration;
use crate::RenderInfo;
use super::frame::Frame;

mod additive_compositor;
mod alpha_compositor;

mod stripes;
mod music_visualizer;

mod flashing_color;
mod solid_color;

mod websocket_input;

mod rotate;

mod temporary;

pub use additive_compositor::AdditiveCompositorEffect;
pub use alpha_compositor::AlphaCompositorEffect;
use enum_dispatch::enum_dispatch;
use reflection::Reflect;
use serde::{Deserialize, Serialize};
pub use stripes::StripeEffect;
pub use music_visualizer::MusicVisualizerEffect;
pub use rotate::RotateEffect;
pub use flashing_color::FlashingColorEffect;
pub use solid_color::SolidColorEffect;
pub use websocket_input::WebsocketInputEffect;

pub use temporary::duration::DurationTemporaryEffect;
pub use temporary::TemporaryEffectCompositor;

/// An effect is a render construct that returns a frame of pixel data with opacity.
/// Effects can take other effects as an input.
#[enum_dispatch]
pub trait Effect {
    fn render(&mut self, delta: Duration, render_info: &mut RenderInfo) -> Frame;
}

/// A temporary effect is a type of effect that determines when it should be removed.
#[enum_dispatch]
pub trait TemporaryEffect {
    fn start(&mut self, render_info: &mut RenderInfo);
    fn is_finished(&self, render_info: &RenderInfo) -> bool;
    fn stop(&mut self, render_info: &mut RenderInfo);
}

// TODO: Maybe we could use [typetag](https://github.com/dtolnay/typetag) instead
// to avoid this enum? I'm not sure if ts-rs will be able to create bindings for it, though.

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[enum_dispatch(Effect)]
pub enum AnyEffect {
    AdditiveCompositor(AdditiveCompositorEffect),
    AlphaCompositor(AlphaCompositorEffect),
    Stripe(StripeEffect),
    MusicVisualizer(MusicVisualizerEffect),
    Rotate(RotateEffect),
    FlashingColor(FlashingColorEffect),
    SolidColor(SolidColorEffect),
    WebsocketInput(WebsocketInputEffect),
}

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[enum_dispatch(TemporaryEffect, Effect)]
pub enum AnyTemporaryEffect {
    TemporaryEffectWrapper(DurationTemporaryEffect),
}