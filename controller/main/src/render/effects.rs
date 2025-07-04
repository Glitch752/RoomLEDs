use crate::RenderInfo;
use super::expressions::ExpressionContext;
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

mod node_editor;

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
pub use node_editor::NodeEditorEffect;

pub use temporary::duration::DurationTemporaryEffect;
pub use temporary::TemporaryEffectCompositor;

/// Context used while rendering that can be changed as state
/// gets passed down to effects.
#[derive(Debug, Clone, Copy)]
pub struct RenderContext {
    pub delta: std::time::Duration,
    /// The time in seconds since the start of the effect.
    pub time: f64,
    pub pixels: u32
}

impl RenderContext {
    pub fn expression_context(&self) -> ExpressionContext {
        return ExpressionContext { current_time: self.time }
    }
}

/// An effect is a render construct that returns a frame of pixel data with opacity.
/// Effects can take other effects as an input.
#[enum_dispatch]
pub trait Effect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame;
}

/// A temporary effect is a type of effect that determines when it should be removed.
#[enum_dispatch]
pub trait TemporaryEffect {
    fn start(&mut self, render_info: &mut RenderInfo);
    fn is_finished(&self, render_info: &RenderInfo) -> bool;
    fn stop(&mut self, render_info: &mut RenderInfo);
}

// Maybe we could use [typetag](https://github.com/dtolnay/typetag) instead
// to avoid this enum? I'm not sure if how to create bindings for it, though.
// This is pretty ergonomic for now.

/// A wrapper for any effect that can be rendered.
/// Used for serialization and deserialization.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[reflect(export_runtime_schema)]
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
    NodeEditorEffect(NodeEditorEffect)
}

/// A wrapper for any temporary effect that can be rendered.
/// Used for serialization and deserialization.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[reflect(export_runtime_schema)]
#[enum_dispatch(TemporaryEffect, Effect)]
pub enum AnyTemporaryEffect {
    TemporaryEffectWrapper(DurationTemporaryEffect),
}