use std::fmt::Display;

#[derive(Debug)]
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
}

pub struct Token {
    _type: TokenType,
    _lexeme: String,
    _literal: Option<String>,
}

impl Token {
    pub fn new(_type: TokenType, _lexeme: String) -> Token {
        Token {
            _type,
            _lexeme,
            _literal: None,
        }
    }

    pub fn get_token(c: char, chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Token> {
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
            _ => return None,
        };
        Some(Token::new(token_type, lexeme.to_string()))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self._type,
            self._lexeme,
            self._literal.clone().unwrap_or("null".to_string())
        )
    }
}
