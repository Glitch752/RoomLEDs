use reflection::Reflect;
use serde::{Deserialize, Serialize};

use super::{AnyExpression, Expression};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct AddExpression {
    pub left: Box<AnyExpression>,
    pub right: Box<AnyExpression>
}

impl Expression for AddExpression {
    fn compute<'a>(&mut self, context: &'a super::ExpressionContext) -> f64 {
        return self.left.compute(&context) + self.right.compute(&context);
    }
}

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct DivideExpression {
    pub left: Box<AnyExpression>,
    pub right: Box<AnyExpression>
}

impl Expression for DivideExpression {
    fn compute<'a>(&mut self, context: &'a super::ExpressionContext) -> f64 {
        return self.left.compute(&context) / self.right.compute(&context);
    }
}

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct MultiplyExpression {
    pub left: Box<AnyExpression>,
    pub right: Box<AnyExpression>
}

impl Expression for MultiplyExpression {
    fn compute<'a>(&mut self, context: &'a super::ExpressionContext) -> f64 {
        return self.left.compute(&context) * self.right.compute(&context);
    }
}


#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct SubtractExpression {
    pub left: Box<AnyExpression>,
    pub right: Box<AnyExpression>
}

impl Expression for SubtractExpression {
    fn compute<'a>(&mut self, context: &'a super::ExpressionContext) -> f64 {
        return self.left.compute(&context) - self.right.compute(&context);
    }
}