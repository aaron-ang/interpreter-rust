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
            Value::Number(n) => write!(f, "{}", remove_trailing_zeros(n)),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
        }
    }
}

fn remove_trailing_zeros(n: &f64) -> String {
    let y = (n * 100_000_000.0).round() / 100_000_000.0;
    format!("{}", y)
}

pub fn eval(expr: &Expr) -> Result<Value, &'static str> {
    let value = match expr {
        Expr::Nil => Value::Nil,
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Number(n) => Value::Number(n.parse().unwrap()),
        Expr::String(s) => Value::String(s.to_string()),
        Expr::Group(expr) => eval(expr)?,
        Expr::Unary(op, expr) => {
            let value = eval(expr)?;
            match op.token_type {
                TokenType::BANG => match value {
                    Value::Bool(b) => Value::Bool(!b),
                    Value::Number(n) => Value::Bool(n == 0.0),
                    Value::String(s) => Value::Bool(s.is_empty()),
                    Value::Nil => Value::Bool(true),
                },
                TokenType::MINUS => match value {
                    Value::Number(n) => Value::Number(-n),
                    _ => return Err("Operand must be a number."),
                },
                _ => unreachable!(),
            }
        }
        Expr::Binary(op, left, right) => {
            let left = eval(left)?;
            let right = eval(right)?;
            match op.token_type {
                TokenType::STAR => match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
                    _ => return Err("Operands must be numbers."),
                },
                TokenType::SLASH => match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Value::Number(l / r),
                    _ => return Err("Operands must be numbers."),
                },
                TokenType::PLUS => match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                    (Value::String(l), Value::String(r)) => Value::String(format!("{}{}", l, r)),
                    _ => return Err("Operands must be two numbers or two strings."),
                },
                TokenType::MINUS => match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
                    _ => return Err("Operands must be numbers."),
                },
                TokenType::LESS
                | TokenType::LESS_EQUAL
                | TokenType::GREATER
                | TokenType::GREATER_EQUAL => match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        Value::Bool(compare_number(&op.token_type, l, r))
                    }
                    _ => return Err("Operands must be numbers."),
                },
                TokenType::EQUAL_EQUAL | TokenType::BANG_EQUAL => {
                    if !variant_eq(&left, &right) {
                        Value::Bool(false)
                    } else {
                        match (left, right) {
                            (Value::Number(l), Value::Number(r)) => {
                                Value::Bool(compare_number(&op.token_type, l, r))
                            }
                            (Value::String(l), Value::String(r)) => {
                                Value::Bool(compare_string(&op.token_type, l, r))
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => todo!(),
            }
        }
    };
    Ok(value)
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn compare_number(op: &TokenType, l: f64, r: f64) -> bool {
    match op {
        TokenType::EQUAL_EQUAL => l == r,
        TokenType::BANG_EQUAL => l != r,
        TokenType::LESS => l < r,
        TokenType::LESS_EQUAL => l <= r,
        TokenType::GREATER => l > r,
        TokenType::GREATER_EQUAL => l >= r,
        _ => unreachable!(),
    }
}

fn compare_string(op: &TokenType, l: String, r: String) -> bool {
    match op {
        TokenType::EQUAL_EQUAL => l == r,
        TokenType::BANG_EQUAL => l != r,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_zeros() {
        assert_eq!(remove_trailing_zeros(&12.40), "12.4");
        assert_eq!(remove_trailing_zeros(&0.0), "0");
    }
}
