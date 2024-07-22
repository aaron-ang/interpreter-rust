use std::iter::Peekable;
use std::slice::Iter;

use crate::{
    exit,
    expr::Expr,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Vec<Expr> {
        let mut exprs = vec![];
        let mut tokens = self.tokens.iter().peekable();
        while let Some(token) = tokens.next() {
            let expr = get_expr(token, &mut tokens);
            exprs.push(expr);
        }
        exprs
    }
}

fn get_expr(token: &Token, tokens: &mut Peekable<Iter<Token>>) -> Expr {
    let expr = match token.token_type {
        TokenType::TRUE => Expr::Bool(true),
        TokenType::FALSE => Expr::Bool(false),
        TokenType::NUMBER => Expr::Number(token.literal.clone().unwrap()),
        TokenType::STRING => Expr::String(token.literal.clone().unwrap()),
        TokenType::LEFT_PAREN => {
            let mut group = vec![];
            while let Some(token) = tokens.next() {
                if token.token_type == TokenType::RIGHT_PAREN {
                    tokens.next();
                    break;
                }
                if tokens.peek().is_none() {
                    eprintln!("Error: Unmatched parentheses.");
                    exit(65);
                }
                group.push(get_expr(token, tokens));
            }
            if group.is_empty() {
                eprintln!("Error: Missing expression in parentheses.");
                exit(65);
            }
            Expr::Group(group)
        }
        TokenType::BANG | TokenType::MINUS => Expr::Unary(
            token.clone(),
            Box::new(get_expr(tokens.next().unwrap(), tokens)),
        ),
        TokenType::NIL => Expr::Nil,
        _ => todo!(),
    };
    expr
}
