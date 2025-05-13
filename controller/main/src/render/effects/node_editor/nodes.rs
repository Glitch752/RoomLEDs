use std::collections::VecDeque;

use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::render::frame::{Frame, PixelColor};

use super::{types::{AnyType, BoolValue, ColorValue, FloatValue, FrameValue, IntegerValue, TryConvert, TryConvertBack}, Node};

// Insane macros

macro_rules! implement_literal_node {
    ($name:ident, $value_type:ty, $var_type:ident) => {
        #[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
        pub struct $name {
            pub value: $value_type
        }

        implement_node!($name, (), ($var_type,), |node: &mut $name, _input| (node.value.clone().into(),));

        impl_try_convert!($var_type);
    }
}

implement_literal_node!(FloatNode, f64, FloatValue);
implement_literal_node!(IntegerNode, i32, IntegerValue);
implement_literal_node!(BoolNode, bool, BoolValue);
implement_literal_node!(ColorNode, PixelColor, ColorValue);
implement_literal_node!(FrameNode, Frame, FrameValue);