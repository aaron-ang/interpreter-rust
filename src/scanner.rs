use crate::grammar::{Literal, Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub error: bool,
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        Scanner {
            source: input.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '=' | '!' | '<' | '>' => self.handle_comparison(c),
            '/' => self.handle_slash(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.handle_string(),
            c if c.is_ascii_digit() => self.handle_number(),
            c if c.is_alphabetic() || c == '_' => self.handle_identifier(),
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                self.error = true;
            }
        };
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.substr(self.start, self.current),
            literal,
            line: self.line,
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
        if self.match_('=') {
            self.add_token(double_char_token);
        } else {
            self.add_token(single_char_token);
        }
    }

    fn handle_slash(&mut self) {
        if self.match_('/') {
            self.advance_end_of_line();
        } else {
            self.add_token(TokenType::SLASH);
        }
    }

    fn advance_end_of_line(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.error = true;
            return;
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes
        let literal = self.substr(self.start + 1, self.current - 1);
        self.add_token_with_literal(TokenType::STRING, Some(Literal::String(literal)))
    }

    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number: f64 = self.substr(self.start, self.current).parse().unwrap();
        self.add_token_with_literal(TokenType::NUMBER, Some(Literal::Number(number)));
    }

    fn handle_identifier(&mut self) {
        while Self::is_identifier_char(self.peek()) {
            self.advance();
        }
        let text = self.substr(self.start, self.current);
        let token_type = TokenType::get_token_type(&text);
        self.add_token(token_type)
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn substr(&self, start: usize, end: usize) -> String {
        self.source[start..end].iter().collect()
    }

    fn match_(&mut self, expected: char) -> bool {
        let is_match = self.peek() == expected;
        if is_match {
            self.advance();
        }
        is_match
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }
}
