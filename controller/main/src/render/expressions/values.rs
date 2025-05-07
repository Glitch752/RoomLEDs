use reflection::Reflect;
use serde::{Deserialize, Serialize};

use super::{AnyExpression, Expression};

/// A constant value.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct LiteralExpression {
    pub value: f64
}

impl LiteralExpression {
    pub fn new(value: f64) -> AnyExpression {
        AnyExpression::Number(LiteralExpression { value })
    }
}

impl Expression for LiteralExpression {
    fn compute<'a>(&mut self, _context: &'a super::ExpressionContext) -> f64 {
        return self.value;
    }
}

/// Gets the current time in seconds.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct CurrentTimeExpression {
}

impl CurrentTimeExpression {
    pub fn new() -> AnyExpression {
        AnyExpression::CurrentTime(CurrentTimeExpression {})
    }
}

impl Expression for CurrentTimeExpression {
    fn compute<'a>(&mut self, context: &'a super::ExpressionContext) -> f64 {
        return context.current_time;
    }
}