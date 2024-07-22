use std::iter::Peekable;
use std::slice::Iter;

use crate::{
    exit,
    expr::Expr,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    exprs: Vec<Expr>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            exprs: vec![],
        }
    }

    pub fn parse(&mut self) -> &Vec<Expr> {
        let mut tokens = self.tokens.iter().peekable();
        while let Some(token) = tokens.next() {
            let expr = self.get_expr(token, &mut tokens);
            self.exprs.push(expr);
        }
        &self.exprs
    }

    fn get_expr(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) -> Expr {
        let expr = match token.token_type {
            TokenType::TRUE => Expr::Bool(true),
            TokenType::FALSE => Expr::Bool(false),
            TokenType::NUMBER => Expr::Number(token.literal.clone().unwrap()),
            TokenType::STRING => Expr::String(token.literal.clone().unwrap()),
            TokenType::LEFT_PAREN => {
                while let Some(token) = tokens.next() {
                    if token.token_type == TokenType::RIGHT_PAREN {
                        tokens.next();
                        break;
                    }
                    if tokens.peek().is_none() {
                        eprintln!("Error: Unmatched parentheses.");
                        exit(65);
                    }
                    let expr = self.get_expr(token, tokens);
                    self.exprs.push(expr);
                }
                if self.exprs.is_empty() {
                    eprintln!("Error: Missing expression in parentheses.");
                    exit(65);
                }
                Expr::Group(self.exprs.drain(..).collect())
            }
            TokenType::BANG | TokenType::MINUS => Expr::Unary(
                token.clone(),
                Box::new(self.get_expr(tokens.next().unwrap(), tokens)),
            ),
            TokenType::STAR | TokenType::SLASH => {
                let left = self.exprs.pop().unwrap();
                let right = self.get_expr(tokens.next().unwrap(), tokens);
                Expr::Binary(token.clone(), Box::new(left), Box::new(right))
            }
            TokenType::NIL => Expr::Nil,
            _ => todo!(),
        };
        expr
    }
}
