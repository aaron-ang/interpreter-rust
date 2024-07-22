use std::fmt::Display;

use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Bool(bool),
    Number(String),
    String(String),
    Group(Vec<Expr>),
    Unary(Token, Box<Expr>),
    Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::Number(n) => write!(f, "{n}"),
            Expr::String(s) => write!(f, "{s}"),
            Expr::Group(g) => {
                write!(
                    f,
                    "(group {})",
                    g.iter()
                        .map(|e| format!("{e}"))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Expr::Unary(opertr, expr) => {
                write!(f, "({} {})", opertr.lexeme, expr)
            }
            Expr::Nil => write!(f, "nil"),
        }
    }
}
