use reflection::Reflect;
use serde::{Deserialize, Serialize};

use super::{AnyExpression, Expression};

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct RoundExpression {
    pub number: Box<AnyExpression>
}

impl Expression for RoundExpression {
    fn compute<'a>(&mut self, context: &'a super::ExpressionContext) -> f64 {
        return self.number.compute(context).round();
    }
}

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct CeilExpression {
    pub number: Box<AnyExpression>
}

impl Expression for CeilExpression {
    fn compute<'a>(&mut self, context: &'a super::ExpressionContext) -> f64 {
        return self.number.compute(context).ceil();
    }
}


#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct FloorExpression {
    pub number: Box<AnyExpression>
}

impl Expression for FloorExpression {
    fn compute<'a>(&mut self, context: &'a super::ExpressionContext) -> f64 {
        return self.number.compute(context).floor();
    }
}