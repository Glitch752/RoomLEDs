use std::collections::VecDeque;

use reflection::Reflect;
use serde::{Deserialize, Serialize};

use super::{types::{AnyType, FloatValue, TryConvertBack, TryConvert}, Node};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct FloatNode {
    pub value: f64
}

impl Node for FloatNode {
    fn compute(&mut self, inputs: VecDeque<AnyType>) -> Result<Vec<AnyType>, String> {
        let input: () = inputs.try_convert()?;
        let result: (FloatValue,) = (self.value.into(),);
        return Ok(result.try_convert_back());
    }
}