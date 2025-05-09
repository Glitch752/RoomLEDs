use crate::render::frame::{Frame, PixelColor};
use std::collections::VecDeque;

pub trait Type {
    fn upcast(self) -> AnyType;
}
#[derive(Clone, Debug, Copy)]
pub struct FloatValue(f32);
#[derive(Clone, Debug, Copy)]
pub struct IntegerValue(i32);
#[derive(Clone, Debug, Copy)]
pub struct BoolValue(bool);
#[derive(Clone, Debug)]
pub struct ColorValue(PixelColor);
#[derive(Clone, Debug)]
pub struct FrameValue(Frame);

impl Type for FloatValue {
    fn upcast(self) -> AnyType {
        AnyType::FloatValue(self)
    }
}
impl Type for BoolValue {
    fn upcast(self) -> AnyType {
        AnyType::BoolValue(self)
    }
}
impl Type for ColorValue {
    fn upcast(self) -> AnyType {
        AnyType::ColorValue(self)
    }
}
impl Type for FrameValue {
    fn upcast(self) -> AnyType {
        AnyType::FrameValue(self)
    }
}
impl Type for IntegerValue {
    fn upcast(self) -> AnyType {
        AnyType::IntegerValue(self)
    }
}

#[derive(Clone, Debug)]
pub enum AnyType {
    FloatValue(FloatValue),
    BoolValue(BoolValue),
    ColorValue(ColorValue),
    FrameValue(FrameValue),
    IntegerValue(IntegerValue)
}

impl AnyType {
    fn type_name(&self) -> String {
        match self {
            AnyType::FloatValue(_) => "flaot".to_string(),
            AnyType::BoolValue(_) => "boolean".to_string(),
            AnyType::ColorValue(_) => "color".to_string(),
            AnyType::FrameValue(_) => "frame".to_string(),
            AnyType::IntegerValue(_) => "integer".to_string()
        }
    }
}

/// Converts a vector of AnyType to a predefined tuple of specific types.  
/// Used to move error-checking from the nodes themselves to the evaluator.
pub trait TryConvert {
    type Output;
    fn try_convert(self, value: VecDeque<AnyType>) -> Result<Self::Output, String>;
}

macro_rules! impl_try_convert {
    ($($name:ident),+) => {
        impl TryConvert for ($($name,)*) {
            type Output = ($($name,)*);
            fn try_convert(self, mut value: VecDeque<AnyType>) -> Result<Self::Output, String> {
                let value_count = [$(stringify!($name),)*].len();
                if value.len() != value_count {
                    return Err(format!("Expected {} values, got {}", value_count, value.len()));
                }
                let result = ($(
                    match value.pop_front() {
                        Some(AnyType::$name(v)) => v,
                        Some(v) => return Err(format!("Expected {}, got {}", stringify!($name), v.type_name())),
                        None => unreachable!(),
                    },
                )*);
                Ok(result)
            }
        }
    };
}

impl_try_convert!(FloatValue);