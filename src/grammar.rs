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

impl TokenType {
    pub fn get_token_type(identifier: &str) -> Self {
        match identifier {
            "and" => Self::AND,
            "class" => Self::CLASS,
            "else" => Self::ELSE,
            "false" => Self::FALSE,
            "for" => Self::FOR,
            "fun" => Self::FUN,
            "if" => Self::IF,
            "nil" => Self::NIL,
            "or" => Self::OR,
            "print" => Self::PRINT,
            "return" => Self::RETURN,
            "super" => Self::SUPER,
            "this" => Self::THIS,
            "true" => Self::TRUE,
            "var" => Self::VAR,
            "while" => Self::WHILE,
            _ => Self::IDENTIFIER,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line_num: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            Some(value) => write!(f, "{:?} {} {value}", self.token_type, self.lexeme),
            None => write!(f, "{:?} {} null", self.token_type, self.lexeme),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Boolean(bool),
    String(String),
    Number(f64),
    Nil,
}

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Boolean(b) => *b,
            Literal::Nil => false,
            _ => true,
        }
    }
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
    Assign {
        name: Token,
        value: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },
    Group(Box<Expression>),
    Literal(Literal),
    Logical {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },
    Unary {
        op: Token,
        right: Box<Expression>,
    },
    Variable(Token),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Assign { name, value } => {
                write!(f, "(assign {} {})", name.lexeme, value)
            }
            Expression::Binary { left, op, right } => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expression::Group(g) => {
                write!(f, "(group {g})")
            }
            Expression::Literal(l) => write!(f, "{l}"),
            Expression::Logical { left, op, right } => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expression::Unary { op, right } => {
                write!(f, "({} {})", op.lexeme, right)
            }
            Expression::Variable(name) => write!(f, "(var {})", name.lexeme),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Vec<Statement>),
    Expression(Expression),
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Print(Expression),
    Variable {
        name: Token,
        init: Option<Expression>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
}
