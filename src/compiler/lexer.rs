use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TokenKind {
    Symbol(Symbol),
    Identifier(String),
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Symbol {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEq,
    Eq,
    EqEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
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

pub struct Tokens<'a> {
    src: &'a str,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
