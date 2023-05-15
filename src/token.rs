use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Structure {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    SemiColon,
}

impl Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Structure::LeftParen => f.write_char('('),
            Structure::RightParen => f.write_char(')'),
            Structure::LeftBrace => f.write_char('{'),
            Structure::RightBrace => f.write_char('}'),
            Structure::Comma => f.write_char(','),
            Structure::Dot => f.write_char('.'),
            Structure::SemiColon => f.write_char(';'),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    /// "-"
    Minus,
    /// "+"
    Plus,
    /// "/"
    Slash,
    /// "*"
    Star,
    /// "!"
    Bang,
    /// "!="
    BangEqual,
    /// "="
    Equal,
    /// "=="
    EqualEqual,
    /// ">"
    Greater,
    /// ">="
    GreaterEqual,
    /// "<"
    Less,
    /// "<="
    LessEqual,
    /// "and"
    And,
    /// "or"
    Or,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Minus => f.write_char('-'),
            Operator::Plus => f.write_char('+'),
            Operator::Slash => f.write_char('/'),
            Operator::Star => f.write_char('*'),
            Operator::Bang => f.write_char('!'),
            Operator::BangEqual => f.write_str("!="),
            Operator::Equal => f.write_char('='),
            Operator::EqualEqual => f.write_str("=="),
            Operator::Greater => f.write_char('>'),
            Operator::GreaterEqual => f.write_str(">="),
            Operator::Less => f.write_char('<'),
            Operator::LessEqual => f.write_str("<="),
            Operator::And => f.write_str("and"),
            Operator::Or => f.write_str("or"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keyword {
    Class,
    Else,
    Fun,
    For,
    If,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Class => f.write_str("class"),
            Keyword::Else => f.write_str("else"),
            Keyword::Fun => f.write_str("fun"),
            Keyword::For => f.write_str("for"),
            Keyword::If => f.write_str("if"),
            Keyword::Print => f.write_str("print"),
            Keyword::Return => f.write_str("return"),
            Keyword::Super => f.write_str("super"),
            Keyword::This => f.write_str("this"),
            Keyword::Var => f.write_str("var"),
            Keyword::While => f.write_str("while"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind<'a> {
    Identifier(&'a str),
    String(&'a str),
    Number(f64),
    Literal(Literal),
    Structure(Structure),
    Operator(Operator),
    Keyword(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    True, False, Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::True => f.write_str("true"),
            Literal::False => f.write_str("false"),
            Literal::Nil => f.write_str("nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenMeta {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub meta: TokenMeta,
}
