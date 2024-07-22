use std::fmt::Display;

pub enum Expr {
    Bool(bool),
    Number(String),
    String(String),
    Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::String(s) => write!(f, "{s}"),
            Expr::Number(n) => write!(f, "{n}"),
            Expr::Nil => write!(f, "nil"),
        }
    }
}
