use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
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
    SLASH,
    STAR,

    EQUAL,
    EQUAL_EQUAL,
    BANG,
    BANG_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,

    COMMENT,

    IDENTIFIER,
    STRING,
    NUMBER,

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

    EOF,
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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line_num: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line_num: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line_num,
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
            '=' | '!' | '<' | '>' => {
                let (single_char_token, double_char_token) = match c {
                    '=' => (TokenType::EQUAL, TokenType::EQUAL_EQUAL),
                    '!' => (TokenType::BANG, TokenType::BANG_EQUAL),
                    '<' => (TokenType::LESS, TokenType::LESS_EQUAL),
                    '>' => (TokenType::GREATER, TokenType::GREATER_EQUAL),
                    _ => unreachable!(),
                };

                if chars.peek() == Some(&'=') {
                    chars.next();
                    (double_char_token, format!("{}=", c), None)
                } else {
                    (single_char_token, c.to_string(), None)
                }
            }
            '/' => match chars.peek() {
                Some('/') => (TokenType::COMMENT, "//".to_string(), None),
                _ => (TokenType::SLASH, c.to_string(), None),
            },
            '"' => {
                let mut lexeme = String::from(c);
                while let Some(c) = chars.next() {
                    lexeme.push(c);
                    if c == '"' {
                        break;
                    }
                }
                if lexeme.chars().last() != Some('"') {
                    eprintln!("[line {}] Error: Unterminated string.", line_num);
                    return None;
                }
                // remove quotes
                let literal = lexeme[1..lexeme.len() - 1].to_string();
                (TokenType::STRING, lexeme, Some(literal))
            }
            c if c.is_ascii_digit() => {
                let mut lexeme = String::from(c);
                let mut has_dot = false;
                let mut peekable = chars.clone().peekable();

                while let Some(next_char) = peekable.next() {
                    match next_char {
                        '0'..='9' => {
                            lexeme.push(next_char);
                            chars.next();
                        }
                        '.' if !has_dot && peekable.peek().is_some_and(|p| p.is_ascii_digit()) => {
                            lexeme.push(next_char);
                            has_dot = true;
                            chars.next();
                        }
                        _ => break,
                    }
                }

                let mut literal = lexeme.clone();
                if literal.chars().last().is_some_and(|c| c.is_ascii_digit()) {
                    if !has_dot {
                        literal.push('.');
                        literal.push('0');
                    } else {
                        while literal.chars().last() == Some('0')
                            && literal.chars().nth_back(1) == Some('0')
                        {
                            literal.pop();
                        }
                    }
                }
                (TokenType::NUMBER, lexeme, Some(literal))
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

        Some(Token::new(token_type, lexeme, literal, line_num))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.as_ref().unwrap_or(&"null".to_string())
        )
    }
}
