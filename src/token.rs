use std::fmt::Display;

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,

    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,

    EQUAL,
    EQUAL_EQUAL,

    BANG,
    BANG_EQUAL,

    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,

    SLASH,
    COMMENT,

    STRING,
}

pub struct Token {
    pub token_type: TokenType,
    _lexeme: String,
    _literal: Option<String>,
}

impl Token {
    pub fn new(_type: TokenType, _lexeme: String) -> Token {
        Token {
            token_type: _type,
            _lexeme,
            _literal: None,
        }
    }

    pub fn new_with_literal(token_type: TokenType, lexeme: String, literal: String) -> Token {
        Token {
            token_type,
            _lexeme: lexeme,
            _literal: Some(literal),
        }
    }

    pub fn get_token(
        c: char,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        line_num: usize,
    ) -> Option<Token> {
        let (token_type, lexeme) = match c {
            '(' => (TokenType::LEFT_PAREN, c.to_string()),
            ')' => (TokenType::RIGHT_PAREN, c.to_string()),
            '{' => (TokenType::LEFT_BRACE, c.to_string()),
            '}' => (TokenType::RIGHT_BRACE, c.to_string()),
            ',' => (TokenType::COMMA, c.to_string()),
            '.' => (TokenType::DOT, c.to_string()),
            '-' => (TokenType::MINUS, c.to_string()),
            '+' => (TokenType::PLUS, c.to_string()),
            ';' => (TokenType::SEMICOLON, c.to_string()),
            '*' => (TokenType::STAR, c.to_string()),
            '=' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    (TokenType::EQUAL_EQUAL, "==".to_string())
                }
                _ => (TokenType::EQUAL, c.to_string()),
            },
            '!' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    (TokenType::BANG_EQUAL, "!=".to_string())
                }
                _ => (TokenType::BANG, c.to_string()),
            },
            '<' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    (TokenType::LESS_EQUAL, "<=".to_string())
                }
                _ => (TokenType::LESS, c.to_string()),
            },
            '>' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    (TokenType::GREATER_EQUAL, ">=".to_string())
                }
                _ => (TokenType::GREATER, c.to_string()),
            },
            '/' => match chars.peek() {
                Some('/') => (TokenType::COMMENT, "//".to_string()),
                _ => (TokenType::SLASH, c.to_string()),
            },
            '"' => {
                let mut string = String::new();
                string.push(c);
                while let Some(c) = chars.next() {
                    string.push(c);
                    if c == '"' {
                        break;
                    }
                }
                if string.chars().last() != Some('"') {
                    eprintln!("[line {}] Error: Unterminated string.", line_num);
                    return None;
                }
                (TokenType::STRING, string)
            }
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line_num, c);
                return None;
            }
        };

        if token_type == TokenType::STRING {
            Some(Token::new_with_literal(
                token_type,
                lexeme.clone(),
                lexeme[1..lexeme.len() - 1].to_string(),
            ))
        } else {
            Some(Token::new(token_type, lexeme))
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self._lexeme,
            self._literal.clone().unwrap_or("null".to_string())
        )
    }
}
