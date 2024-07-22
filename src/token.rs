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
    NUMBER,

    IDENTIFIER,

    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}

pub fn get_token_type(identifier: &str) -> TokenType {
    match identifier {
        "and" => TokenType::AND,
        "class" => TokenType::CLASS,
        "else" => TokenType::ELSE,
        "false" => TokenType::FALSE,
        "for" => TokenType::FOR,
        "fun" => TokenType::FUN,
        "if" => TokenType::IF,
        "nil" => TokenType::NIL,
        "or" => TokenType::OR,
        "print" => TokenType::PRINT,
        "return" => TokenType::RETURN,
        "super" => TokenType::SUPER,
        "this" => TokenType::THIS,
        "true" => TokenType::TRUE,
        "var" => TokenType::VAR,
        "while" => TokenType::WHILE,
        _ => TokenType::IDENTIFIER,
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    _literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, _literal: Option<String>) -> Token {
        Token {
            token_type,
            lexeme,
            _literal,
        }
    }

    pub fn get_token(
        c: char,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        line_num: usize,
    ) -> Option<Token> {
        let (token_type, lexeme, literal): (TokenType, String, Option<String>) = match c {
            '(' => (TokenType::LEFT_PAREN, c.to_string(), None),
            ')' => (TokenType::RIGHT_PAREN, c.to_string(), None),
            '{' => (TokenType::LEFT_BRACE, c.to_string(), None),
            '}' => (TokenType::RIGHT_BRACE, c.to_string(), None),
            ',' => (TokenType::COMMA, c.to_string(), None),
            '.' => (TokenType::DOT, c.to_string(), None),
            '-' => (TokenType::MINUS, c.to_string(), None),
            '+' => (TokenType::PLUS, c.to_string(), None),
            ';' => (TokenType::SEMICOLON, c.to_string(), None),
            '*' => (TokenType::STAR, c.to_string(), None),
            '=' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    (TokenType::EQUAL_EQUAL, "==".to_string(), None)
                }
                _ => (TokenType::EQUAL, c.to_string(), None),
            },
            '!' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    (TokenType::BANG_EQUAL, "!=".to_string(), None)
                }
                _ => (TokenType::BANG, c.to_string(), None),
            },
            '<' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    (TokenType::LESS_EQUAL, "<=".to_string(), None)
                }
                _ => (TokenType::LESS, c.to_string(), None),
            },
            '>' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    (TokenType::GREATER_EQUAL, ">=".to_string(), None)
                }
                _ => (TokenType::GREATER, c.to_string(), None),
            },
            '/' => match chars.peek() {
                Some('/') => (TokenType::COMMENT, "//".to_string(), None),
                _ => (TokenType::SLASH, c.to_string(), None),
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
                (
                    TokenType::STRING,
                    string.clone(),
                    Some(string[1..string.len() - 1].to_string()),
                )
            }
            c if c.is_ascii_digit() => {
                let mut number = String::from(c);
                let mut has_dot = false;
                let mut peekable = chars.clone().peekable();
                while let Some(p) = peekable.next() {
                    if p.is_ascii_digit() {
                        number.push(p);
                        chars.next();
                    } else {
                        if p == '.'
                            && !has_dot
                            && peekable.peek().is_some_and(|p| p.is_ascii_digit())
                        {
                            number.push(p);
                            chars.next();
                            has_dot = true;
                        } else {
                            break;
                        }
                    }
                }

                if number.chars().last().is_some_and(|c| c.is_ascii_digit()) {
                    let value = number.clone();
                    if !has_dot {
                        number.push('.');
                        number.push('0');
                    } else {
                        while number.chars().last() == Some('0')
                            && number.chars().nth_back(1) == Some('0')
                        {
                            number.pop();
                        }
                    }
                    (TokenType::NUMBER, value, Some(number))
                } else {
                    (TokenType::NUMBER, number.clone(), Some(number))
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut identifier = String::from(c);
                while let Some(p) = chars.peek() {
                    if p.is_alphanumeric() || *p == '_' {
                        identifier.push(*p);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let token_type = get_token_type(&identifier);
                (token_type, identifier, None)
            }
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line_num, c);
                return None;
            }
        };

        Some(Token::new(token_type, lexeme, literal))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            self._literal.clone().unwrap_or("null".to_string())
        )
    }
}
