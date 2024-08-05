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
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

pub struct Evaluator<'a> {
    exprs: &'a Vec<Expr>,
}

impl<'a> Evaluator<'a> {
    pub fn new(exprs: &'a Vec<Expr>) -> Self {
        Self { exprs }
    }

    pub fn evaluate(&self) {
        for expr in self.exprs {
            println!("{}", self.token_to_string(expr));
        }
    }

    fn token_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::String(s) => s.trim_matches('"').to_string(),
            Expr::Number(n) => {
                if n.contains('.') {
                    let trimmed = n.trim_end_matches('0');
                    if trimmed.ends_with('.') {
                        trimmed.trim_end_matches('.').to_string()
                    } else {
                        trimmed.to_string()
                    }
                } else {
                    n.to_string()
                }
            }
            Expr::Group(g) => self.token_to_string(g),
            Expr::Unary(op, expr) => {
                if op.token_type == TokenType::BANG {
                    self.token_to_string(&get_negated_expr(expr))
                } else {
                    format!("{}{}", op.lexeme, self.token_to_string(expr))
                }
            }
            _ => expr.to_string(),
        }
    }
}

fn get_negated_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::Bool(b) => Expr::Bool(!b),
        Expr::Number(n) => Expr::Bool(n == "0.0"),
        Expr::Group(g) => get_negated_expr(g),
        Expr::Unary(op, expr) => {
            if op.token_type == TokenType::BANG {
                get_negated_expr(expr)
            } else {
                *expr.clone()
            }
        }
        Expr::Nil => Expr::Bool(true),
        _ => expr.clone(),
    }
}
