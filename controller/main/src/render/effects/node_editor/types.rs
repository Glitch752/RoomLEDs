use crate::render::frame::{Frame, PixelColor};
use std::collections::VecDeque;

pub trait Type {
    fn upcast(self) -> AnyType;
}
#[derive(Clone, Debug, Copy)]
pub struct FloatValue(f64);
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
impl From<f64> for FloatValue {
    fn from(value: f64) -> Self {
        return Self(value);
    }
}
impl Type for BoolValue {
    fn upcast(self) -> AnyType {
        AnyType::BoolValue(self)
    }
}
impl From<bool> for BoolValue {
    fn from(value: bool) -> Self {
        return Self(value);
    }
}
impl Type for ColorValue {
    fn upcast(self) -> AnyType {
        AnyType::ColorValue(self)
    }
}
impl From<PixelColor> for ColorValue {
    fn from(value: PixelColor) -> Self {
        return Self(value);
    }
}
impl Type for FrameValue {
    fn upcast(self) -> AnyType {
        AnyType::FrameValue(self)
    }
}
impl From<Frame> for FrameValue {
    fn from(value: Frame) -> Self {
        return Self(value);
    }
}
impl Type for IntegerValue {
    fn upcast(self) -> AnyType {
        AnyType::IntegerValue(self)
    }
}
impl From<i32> for IntegerValue {
    fn from(value: i32) -> Self {
        return Self(value);
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
    pub fn type_name(&self) -> String {
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
pub trait TryConvert<T> {
    type Output = T;
    fn try_convert(self) -> Result<T, String>;
}

/// Converts a specialized type to a generic vector of AnyType.
/// Used to move error-checking from the nodes themselves to the evaluator.
pub trait TryConvertBack {
    fn try_convert_back(self) -> Vec<AnyType>;
}

#[macro_export]
macro_rules! impl_try_convert {
    ($(($idx:tt, $name:ident)),+) => {
        impl TryConvert<($($name,)*)> for VecDeque<AnyType> where
            $($name: crate::effects::node_editor::types::Type),* {
            fn try_convert(mut self) -> Result<Self::Output, String> {
                let value_count = [$(stringify!($name),)*].len();
                if self.len() != value_count {
                    return Err(format!("Expected {} values, got {}", value_count, self.len()));
                }
                let result = ($(
                    match self.pop_front() {
                        Some(AnyType::$name(v)) => v,
                        Some(v) => return Err(format!("Expected {}, got {}", stringify!($name), v.type_name())),
                        None => unreachable!(),
                    },
                )*);
                Ok(result)
            }
        }
        impl TryConvertBack for ($($name,)*) {
            fn try_convert_back(self) -> Vec<AnyType> {
                use crate::render::effects::node_editor::types::Type;

                let mut result = Vec::new();
                $(
                    result.push(self.$idx.upcast());
                )*
                result
            }
        }
    };

    // We just have a preset list of parameter counts since it's easiest with declarative macros.
    ($name0:ident) => { impl_try_convert!((0, $name0)); };
    ($name0:ident, $name1:ident) => { impl_try_convert!((0, $name0), (1, $name1)); };
    ($name0:ident, $name1:ident, $name2:ident) => { impl_try_convert!((0, $name0), (1, $name1), (2, $name2)); };
    ($name0:ident, $name1:ident, $name2:ident, $name3:ident) => { impl_try_convert!((0, $name0), (1, $name1), (2, $name2), (3, $name3)); };
    ($name0:ident, $name1:ident, $name2:ident, $name3:ident, $name4:ident) => { impl_try_convert!((0, $name0), (1, $name1), (2, $name2), (3, $name3), (4, $name4)); };
    ($name0:ident, $name1:ident, $name2:ident, $name3:ident, $name4:ident, $name5:ident) => { impl_try_convert!((0, $name0), (1, $name1), (2, $name2), (3, $name3), (4, $name4), (5, $name5)); };
    ($name0:ident, $name1:ident, $name2:ident, $name3:ident, $name4:ident, $name5:ident, $name6:ident) => { impl_try_convert!((0, $name0), (1, $name1), (2, $name2), (3, $name3), (4, $name4), (5, $name5), (6, $name6)); };
    ($name0:ident, $name1:ident, $name2:ident, $name3:ident, $name4:ident, $name5:ident, $name6:ident, $name7:ident) => { impl_try_convert!((0, $name0), (1, $name1), (2, $name2), (3, $name3), (4, $name4), (5, $name5), (6, $name6), (7, $name7)); };
    ($name0:ident, $name1:ident, $name2:ident, $name3:ident, $name4:ident, $name5:ident, $name6:ident, $name7:ident, $name8:ident) => { impl_try_convert!((0, $name0), (1, $name1), (2, $name2), (3, $name3), (4, $name4), (5, $name5), (6, $name6), (7, $name7), (8, $name8)); };
}

// Manual implementation for empty tuples
impl TryConvert<()> for VecDeque<AnyType> {
    fn try_convert(self) -> Result<Self::Output, String> {
        if self.len() != 0 {
            return Err(format!("Expected 0 values, got {}", self.len()));
        }
        Ok(())
    }
}