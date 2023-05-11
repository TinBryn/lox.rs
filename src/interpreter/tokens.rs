use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keyword {
    // Structure tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    SemiColon,

    // Arithmatic
    Minus,
    Plus,
    Slash,
    Star,

    // Logical
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Word operators
    And,
    Or,

    // Keywords
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::LeftParen => f.write_char('('),
            Keyword::RightParen => f.write_char(')'),
            Keyword::LeftBrace => f.write_char('{'),
            Keyword::RightBrace => f.write_char('}'),
            Keyword::Comma => f.write_char(','),
            Keyword::Dot => f.write_char('.'),
            Keyword::SemiColon => f.write_char(';'),
            Keyword::Minus => f.write_char('-'),
            Keyword::Plus => f.write_char('+'),
            Keyword::Slash => f.write_char('/'),
            Keyword::Star => f.write_char('*'),
            Keyword::Bang => f.write_char('!'),
            Keyword::BangEqual => f.write_str("!="),
            Keyword::Equal => f.write_char('='),
            Keyword::EqualEqual => f.write_str("=="),
            Keyword::Greater => f.write_char('>'),
            Keyword::GreaterEqual => f.write_str(">="),
            Keyword::Less => f.write_char('<'),
            Keyword::LessEqual => f.write_str("<="),
            Keyword::And => f.write_str("and"),
            Keyword::Or => f.write_str("or"),
            Keyword::Class => f.write_str("class"),
            Keyword::Else => f.write_str("else"),
            Keyword::False => f.write_str("false"),
            Keyword::Fun => f.write_str("fun"),
            Keyword::For => f.write_str("for"),
            Keyword::If => f.write_str("if"),
            Keyword::Nil => f.write_str("nil"),
            Keyword::Print => f.write_str("print"),
            Keyword::Return => f.write_str("return"),
            Keyword::Super => f.write_str("super"),
            Keyword::This => f.write_str("this"),
            Keyword::True => f.write_str("true"),
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
    Keyword(Keyword),
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub row: usize,
    pub col: usize,
}
