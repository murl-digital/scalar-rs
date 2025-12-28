#[derive(Debug)]
pub enum Value {
    CurrentField,
    Ident(&'static str),
    Value(serde_json::Value),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Expression {
    Equals {
        lhs: Value,
        rhs: Value,
    },
    NotEquals {
        lhs: Value,
        rhs: Value,
    },
    And {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Or {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

pub use scalar_expr_macro::expression;
pub use serde_json::to_value;
