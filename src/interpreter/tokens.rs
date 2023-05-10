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

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
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
