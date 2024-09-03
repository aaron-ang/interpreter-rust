use crate::grammar::{Literal, Token, TokenType};

pub struct Scanner<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    current: String,
    tokens: Vec<Token>,
    line_num: usize,
    pub error: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner {
            chars: input.chars().peekable(),
            current: String::new(),
            tokens: vec![],
            line_num: 1,
            error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while self.chars.peek().is_some() {
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line_num: self.line_num,
        });
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.chars.next().unwrap();
        self.current = c.to_string();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '=' | '!' | '<' | '>' => self.handle_comparison(c),
            '/' => self.handle_slash(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line_num += 1,
            '"' => self.handle_string(),
            c if c.is_ascii_digit() => self.handle_number(),
            c if c.is_alphabetic() || c == '_' => self.handle_identifier(),
            _ => {
                eprintln!(
                    "[line {}] Error: Unexpected character: {}",
                    self.line_num, c
                );
                self.error = true;
            }
        };
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.current.clone(),
            literal,
            line_num: self.line_num,
        });
    }

    fn handle_comparison(&mut self, c: char) {
        let (single_char_token, double_char_token) = match c {
            '=' => (TokenType::EQUAL, TokenType::EQUAL_EQUAL),
            '!' => (TokenType::BANG, TokenType::BANG_EQUAL),
            '<' => (TokenType::LESS, TokenType::LESS_EQUAL),
            '>' => (TokenType::GREATER, TokenType::GREATER_EQUAL),
            _ => unreachable!(),
        };
        if self.chars.peek() == Some(&'=') {
            self.current.push(self.chars.next().unwrap());
            self.add_token(double_char_token, None);
        } else {
            self.add_token(single_char_token, None);
        }
    }

    fn handle_slash(&mut self) {
        if self.chars.peek() == Some(&'/') {
            self.advance_next_line();
        } else {
            self.add_token(TokenType::SLASH, None);
        }
    }

    fn advance_next_line(&mut self) {
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                self.line_num += 1;
                break;
            }
        }
    }

    fn handle_string(&mut self) {
        while let Some(c) = self.chars.next() {
            self.current.push(c);
            if c == '"' {
                break;
            }
        }
        if !self.current.ends_with('"') {
            eprintln!("[line {}] Error: Unterminated string.", self.line_num);
            self.error = true;
            return;
        }
        // remove quotes
        let literal = self.current[1..self.current.len() - 1].to_string();
        self.add_token(TokenType::STRING, Some(Literal::String(literal)))
    }

    fn handle_number(&mut self) {
        let mut has_dot = false;
        while let Some(&next_char) = self.chars.peek() {
            match next_char {
                '0'..='9' => {
                    self.current.push(next_char);
                    self.chars.next();
                }
                '.' if !has_dot
                    && self
                        .chars
                        .clone()
                        .nth(1)
                        .is_some_and(|p| p.is_ascii_digit()) =>
                {
                    self.current.push(next_char);
                    has_dot = true;
                    self.chars.next();
                }
                _ => break,
            }
        }
        let number: f64 = self.current.parse().unwrap();
        self.add_token(TokenType::NUMBER, Some(Literal::Number(number)));
    }

    fn handle_identifier(&mut self) {
        while let Some(next_char) = self.chars.peek() {
            if next_char.is_alphanumeric() || *next_char == '_' {
                self.current.push(*next_char);
                self.chars.next();
            } else {
                break;
            }
        }
        let token_type = TokenType::get_token_type(&self.current);
        self.add_token(token_type, None)
    }
}
