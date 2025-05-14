use core::fmt;
use std::{any::TypeId, collections::{HashMap, VecDeque}};

use enum_dispatch::enum_dispatch;
use reflection::Reflect;
use serde::{Deserialize, Serialize};
use types::{AnyType, TryConvert, TryConvertBack};

use crate::{render::frame::Frame, RenderInfo};
use super::{Effect, RenderContext};

#[macro_use]
mod types;

mod nodes;

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

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: &'static str,
    pub type_id: TypeId,
}

impl TypeInfo {
    pub const INT: Self = Self { name: "int", type_id: TypeId::of::<i32>() };
    pub const FLOAT: Self = Self { name: "float", type_id: TypeId::of::<f64>() };
    pub const BOOL: Self = Self { name: "bool", type_id: TypeId::of::<bool>() };
    pub const STRING: Self = Self { name: "string", type_id: TypeId::of::<String>() };
}

#[derive(Debug, Clone)]
pub struct PortInfo {
    pub name: String,
    pub type_info: TypeInfo,
}

pub trait Node {
    fn name(&self) -> &'static str;
    fn input_ports(&self) -> &[PortInfo];
    fn output_ports(&self) -> &[PortInfo];
    fn compute(&mut self, inputs: VecDeque<AnyType>) -> Result<Vec<AnyType>, String>;
}

pub struct TypedNode<I, O> {
    name: &'static str,
    inputs: Vec<PortInfo>,
    outputs: Vec<PortInfo>,
    func: Box<dyn Fn(I) -> Result<O, String> + Send + Sync>,
}

impl<I, O> TypedNode<I, O>
where
    I: TryConvert<I> + 'static,
    O: TryConvertBack + 'static,
{
    pub fn new(
        name: &'static str,
        inputs: Vec<PortInfo>,
        outputs: Vec<PortInfo>,
        func: impl Fn(I) -> Result<O, String> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name,
            inputs,
            outputs,
            func: Box::new(func),
        }
    }
}

impl<I, O> Node for TypedNode<I, O>
where
    VecDeque<AnyType>: TryConvert<I> + 'static,
    O: TryConvertBack + 'static,
{
    fn name(&self) -> &'static str {
        self.name
    }

    fn input_ports(&self) -> &[PortInfo] {
        &self.inputs
    }

    fn output_ports(&self) -> &[PortInfo] {
        &self.outputs
    }

    fn compute(&mut self, inputs: VecDeque<AnyType>) -> Result<Vec<AnyType>, String> {
        let typed_inputs: I = inputs.try_convert()?;
        let output = (self.func)(typed_inputs)?;
        Ok(output.try_convert_back())
    }
}

struct NodeRegistry {
    nodes: HashMap<String, Box<dyn Node + Send + Sync>>,
}

impl NodeRegistry {
    fn new() -> Self {
        let mut nodes = HashMap::new();
        // TODO: Register built-in nodes
        Self { nodes }
    }

    fn get_node(&self, name: &str) -> Option<&Box<dyn Node + Send + Sync>> {
        self.nodes.get(name)
    }
}

struct NodeData {
    node_type: String,
    input_connections: Vec<(NodeID, usize)>,
    output_connections: Vec<(NodeID, usize)>
}

/// An effect that renders a frame based on a node-based graphical editor.
/// This is by far the most complex effect type, as it allows for arbitrary
/// calculations for every pixel in the frame.
pub struct NodeEditorEffect {
    nodes: HashMap<NodeID, NodeData>
}

impl Effect for NodeEditorEffect {
    fn render(&mut self, context: RenderContext, render_info: &mut RenderInfo) -> Frame {
        // TODO
        Frame::empty(context.pixels)
    }
}