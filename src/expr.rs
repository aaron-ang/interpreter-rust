use std::fmt::Display;

pub enum Expr {
    Bool(bool),
    Nil,
    Number(String),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::Nil => write!(f, "nil"),
            Expr::Number(n) => write!(f, "{n}"),
        }
    }
}
