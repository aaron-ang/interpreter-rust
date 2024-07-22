use std::fmt::Display;

#[derive(Debug)]
pub enum Expr {
    Bool(bool),
    Number(String),
    String(String),
    Group(Vec<Expr>),
    Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::Number(n) => write!(f, "{n}"),
            Expr::String(s) => write!(f, "{s}"),
            Expr::Group(g) => {
                write!(f, "(group ")?;
                for expr in g {
                    write!(f, "{expr}")?;
                }
                write!(f, ")")?;
                Ok(())
            }
            Expr::Nil => write!(f, "nil"),
        }
    }
}
