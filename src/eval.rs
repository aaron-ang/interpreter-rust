use std::fmt::Display;

use crate::expr::Expr;
use crate::token::TokenType;

pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
        }
    }
}

pub fn eval(expr: &Expr) -> Value {
    match expr {
        Expr::Nil => Value::Nil,
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Number(n) => Value::Number(n.parse().unwrap()),
        Expr::String(s) => Value::String(s.to_string()),
        Expr::Group(expr) => eval(expr),
        Expr::Unary(op, expr) => {
            let value = eval(expr);
            match op.token_type {
                TokenType::BANG => match value {
                    Value::Bool(b) => Value::Bool(!b),
                    Value::Number(n) => Value::Bool(n == 0.0),
                    Value::String(s) => Value::Bool(s.is_empty()),
                    Value::Nil => Value::Bool(true),
                },
                TokenType::MINUS => match value {
                    Value::Number(n) => Value::Number(-n),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        Expr::Binary(op, left, right) => todo!(),
    }
}
