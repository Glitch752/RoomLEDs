use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use reflection::Reflect;
use serde::{Deserialize, Serialize};
use types::{AnyType, TryConvert, Type};

use crate::{render::frame::{Frame, PixelColor}, RenderInfo};
use super::{Effect, RenderContext};

mod types;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
struct NodeID(uuid::Uuid);

impl NodeID {
    pub fn new() -> NodeID {
        NodeID(uuid::Uuid::new_v4())
    }
}

impl Reflect for NodeID {
    const INLINE: bool = true;
    fn ts_definition() -> String {
        return "string".to_string();
    }
    fn schema() -> reflection::schema::Schema {
        reflection::schema::Schema::String
    }
    fn visit_dependencies(_: &mut impl reflection::TypeVisitor) where Self: 'static {}
}

#[enum_dispatch]
pub trait NodeImplementation {
    type Input: TryConvert;
    type Output: TryConvert;

    fn should_recompute(&self) -> bool {
        return true;
    }
    fn compute(&mut self, inputs: Self::Input) -> Self::Output;
}

#[enum_dispatch(NodeImplementation)]
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum AnyNodeImplementation {
}

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    id: NodeID,
    implementation: AnyNodeImplementation,
    inputs: Vec<(NodeID, usize)>,
    last_frame_rendered: u32,
    #[serde(skip)]
    output_values: Vec<AnyType>
}

impl Node {
    pub fn new(implementation: AnyNodeImplementation) -> Self {
        Self {
            id: NodeID::new(),
            implementation,
            inputs: Vec::new(),
            last_frame_rendered: 0,
            output_values: Vec::new(),
        }
    }
    
}

/// An effect that renders a frame based on a node-based graphical editor.
/// This is by far the most complex effect type, as it allows for arbitrary
/// calculations for every pixel in the frame.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct NodeEditorEffect {
    nodes: HashMap<NodeID, Node>
}

impl Effect for NodeEditorEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
        // TODO
        Frame::empty(context.pixels)
    }
}