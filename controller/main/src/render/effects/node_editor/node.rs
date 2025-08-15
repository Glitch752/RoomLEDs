use std::{collections::VecDeque, sync::Arc};

use dyn_clone::DynClone;
use reflection::Reflect;
use serde::{Deserialize, Serialize};

use super::types::{AnyType, TryConvert, TryConvertBack, TypeInfo};


#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct NodeID(uuid::Uuid);

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

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct PortInfo {
    pub name: String,
    pub type_info: TypeInfo
}

pub trait Node : DynClone + Send + Sync {
    fn name(&self) -> &'static str;
    fn input_ports(&self) -> &[PortInfo];
    fn output_ports(&self) -> &[PortInfo];
    fn compute(&mut self, inputs: VecDeque<AnyType>) -> Result<Vec<AnyType>, String>;
}

/// A simple node with no state or parameters.
/// Types can be inferred from the function types, and type-checking code will be automatically generated!
#[derive(Clone)]
pub struct SimpleTypedNode<I, O> {
    name: &'static str,
    inputs: Vec<PortInfo>,
    outputs: Vec<PortInfo>,
    func: Arc<Box<dyn Fn(I) -> Result<O, String> + Send + Sync>>,
}

impl<I, O> SimpleTypedNode<I, O>
where
    VecDeque<AnyType>: TryConvert<I> + 'static,
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
            func: Arc::new(Box::new(func)),
        }
    }
}

impl<I, O> Node for SimpleTypedNode<I, O>
where
    VecDeque<AnyType>: TryConvert<I> + 'static,
    I: Clone,
    O: Clone + TryConvertBack + 'static,
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