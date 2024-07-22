use std::fmt::Display;

pub enum Expr {
    Bool(bool),
    Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => f.write_fmt(format_args!("{b}")),
            Expr::Nil => f.write_str("nil"),
        }
    }
}
