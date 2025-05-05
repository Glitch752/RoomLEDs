use crate::render::{frame::Frame, RenderInfo};

use super::{AnyTemporaryEffect, Effect, RenderContext, TemporaryEffect};

pub mod duration;

/// A compositor for temporary effects.
/// Manages the lifecycle of temporary effects by starting, running, stopping, and sequentially rendering
/// the next effect.
#[derive(Debug)]
pub struct TemporaryEffectCompositor {
    effects: Vec<Box<AnyTemporaryEffect>>,
    running: bool
}

impl TemporaryEffectCompositor {
    /// Creates a new temporary effect compositor with the specified effects.
    pub fn new(effects: Vec<Box<AnyTemporaryEffect>>) -> Self {
        Self {
            effects,
            running: false
        }
    }

    /// Adds a new effect to the compositor.
    /// The effect will be rendered after all the other effects in the sequence.
    pub fn add_effect(&mut self, effect: AnyTemporaryEffect) {
        self.effects.push(Box::new(effect));
    }
}

impl Effect for TemporaryEffectCompositor {
    /// Renders the current effect in the sequence.
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
        if let Some(effect) = self.effects.first_mut() {
            if !self.running {
                effect.start(render_info);
                self.running = true;
            }

            let frame = effect.render(context, render_info);

            if effect.is_finished(&render_info) {
                effect.stop(render_info);
                self.running = false;
                self.effects.remove(0);
            }

            frame
        } else {
            self.running = false;

            Frame::empty(context.pixels)
        }
    }
}