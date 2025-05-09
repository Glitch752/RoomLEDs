use enum_dispatch::enum_dispatch;
use reflection::Reflect;
pub use arithmetic::{AddExpression, DivideExpression, MultiplyExpression, SubtractExpression};
pub use round::{CeilExpression, FloorExpression, RoundExpression};
pub use values::{CurrentTimeExpression, LiteralExpression};
use serde::{Deserialize, Serialize};

mod values;
mod arithmetic;
mod round;

#[derive(Debug, Clone, Copy)]
pub struct ExpressionContext {
    /// The current time, in seconds.
    pub current_time: f64
}

/// An expression is a tree of constructs that returns a number.
/// Expressions can take other expressions as inputs.
#[enum_dispatch]
pub trait Expression {
    fn compute<'a>(&mut self, context: &'a ExpressionContext) -> f64;
}

/// A wrapper for any expression.
/// Used for serialization and deserialization.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[reflect(export_runtime_schema)]
#[enum_dispatch(Expression)]
pub enum AnyExpression {
    Number(LiteralExpression),
    CurrentTime(CurrentTimeExpression),
    Add(AddExpression),
    Subtract(SubtractExpression),
    Multiply(MultiplyExpression),
    Divide(DivideExpression),
    Round(RoundExpression),
    Floor(FloorExpression),
    Ceil(CeilExpression)
}