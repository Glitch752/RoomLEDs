use std::collections::VecDeque;

use reflection::Reflect;
use serde::{Deserialize, Serialize};

use crate::render::frame::{Frame, PixelColor};

use super::{types::{AnyType, BoolValue, ColorValue, FloatValue, FrameValue, IntegerValue, TryConvert, TryConvertBack}, Node};
