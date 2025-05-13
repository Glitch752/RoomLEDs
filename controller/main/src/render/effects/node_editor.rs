use std::collections::{HashMap, VecDeque};

use enum_dispatch::enum_dispatch;
use reflection::Reflect;
use serde::{Deserialize, Serialize};
use types::AnyType;

use crate::{render::frame::Frame, RenderInfo};
use super::{Effect, RenderContext};

macro_rules! implement_node {
    ($name:ident, $input_type:ty, $output_type:ty, $body:expr) => {
        impl Node for $name {
            fn compute(&mut self, inputs: VecDeque<AnyType>) -> Result<Vec<AnyType>, String> {
                let input: $input_type = inputs.try_convert()?;
                let result: $output_type = $body(self as &mut $name, input);
                return Ok(result.try_convert_back());
            }
        }
    };
}

#[macro_use]
mod types;

mod nodes;

pub use nodes::FloatNode;

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
pub trait Node {
    fn should_recompute(&self) -> bool {
        return true;
    }
    fn compute(&mut self, inputs: VecDeque<AnyType>) -> Result<Vec<AnyType>, String>;
}

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[enum_dispatch(Node)]
enum NodeImplementation {
    Float(FloatNode)
}

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct NodeInstance {
    id: NodeID,
    implementation: NodeImplementation,
    inputs: Vec<(NodeID, usize)>,
    last_frame_rendered: u32,
    #[serde(skip)]
    output_values: Vec<AnyType>
}

impl NodeInstance {
    pub fn new(implementation: NodeImplementation) -> Self {
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
    nodes: HashMap<NodeID, NodeInstance>
}

impl Effect for NodeEditorEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
        // TODO
        Frame::empty(context.pixels)
    }
}