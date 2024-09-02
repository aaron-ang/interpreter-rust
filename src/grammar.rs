use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Literal {
    Boolean(bool),
    String(String),
    Number(f64),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Boolean(b) => write!(f, "{b}"),
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(n) => {
                let int = n.trunc();
                if int == *n {
                    write!(f, "{int}.0")
                } else {
                    write!(f, "{n}")
                }
            }
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Group(Box<Expression>),
    Unary {
        op: Token,
        expr: Box<Expression>,
    },
    Binary {
        op: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(l) => write!(f, "{l}"),
            Expression::Group(g) => {
                write!(f, "(group {g})")
            }
            Expression::Unary { op, expr } => {
                write!(f, "({} {})", op.lexeme, expr)
            }
            Expression::Binary { op, left, right } => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
}
