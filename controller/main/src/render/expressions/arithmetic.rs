use reflection::Reflect;
use serde::{Deserialize, Serialize};

use super::{AnyExpression, Expression};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct AddExpression {
    pub left: Box<AnyExpression>,
    pub right: Box<AnyExpression>
}

impl AddExpression {
    #[allow(unused)]
    pub fn new(left: AnyExpression, right: AnyExpression) -> AnyExpression {
        AnyExpression::Add(AddExpression {
            left: Box::new(left),
            right: Box::new(right)
        })
    }
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

impl DivideExpression {
    #[allow(unused)]
    pub fn new(left: AnyExpression, right: AnyExpression) -> AnyExpression {
        AnyExpression::Divide(DivideExpression {
            left: Box::new(left),
            right: Box::new(right)
        })
    }
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

impl MultiplyExpression {
    #[allow(unused)]
    pub fn new(left: AnyExpression, right: AnyExpression) -> AnyExpression {
        AnyExpression::Multiply(MultiplyExpression {
            left: Box::new(left),
            right: Box::new(right)
        })
    }
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

impl SubtractExpression {
    #[allow(unused)]
    pub fn new(left: AnyExpression, right: AnyExpression) -> AnyExpression {
        AnyExpression::Subtract(SubtractExpression {
            left: Box::new(left),
            right: Box::new(right)
        })
    }
}

impl Expression for SubtractExpression {
    fn compute<'a>(&mut self, context: &'a super::ExpressionContext) -> f64 {
        return self.left.compute(&context) - self.right.compute(&context);
    }
}