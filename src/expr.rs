use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Bool(bool),
    Number(String),
    String(String),
    Group(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Token, Box<Expr>, Box<Expr>),
    Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::Number(n) => write!(f, "{n}"),
            Expr::String(s) => write!(f, "{s}"),
            Expr::Group(g) => {
                write!(f, "(group {g})")
            }
            Expr::Unary(op, expr) => {
                write!(f, "({} {})", op.lexeme, expr)
            }
            Expr::Binary(op, left, right) => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expr::Nil => write!(f, "nil"),
        }
    }
}
