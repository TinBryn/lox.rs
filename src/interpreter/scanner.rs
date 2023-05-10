use std::{iter::Peekable, str::Chars};

use crate::error::LexicalError;

use super::tokens::{Keyword, Token, TokenKind};

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    row: usize,
    col: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            row: 1,
            col: 1,
        }
    }

    fn scan_token(&mut self) -> Option<Result<Token<'a>, LexicalError>> {
        use Keyword::*;
        let (c, row, col) = self.advance()?;
        let sym = loop {
            match c {
                '(' => break LeftParen,
                ')' => break RightParen,
                '{' => break LeftBrace,
                '}' => break RightBrace,
                ',' => break Comma,
                '.' => break Dot,
                '-' => break Minus,
                '+' => break Plus,
                ';' => break SemiColon,
                '*' => break Star,
                '!' => break if self.matches('=') { BangEqual } else { Bang },
                '=' => break if self.matches('=') { EqualEqual } else { Equal },
                '<' => break if self.matches('=') { LessEqual } else { Less },
                '>' => {
                    break if self.matches('=') {
                        GreaterEqual
                    } else {
                        Greater
                    }
                }
                '/' => {
                    if self.matches('/') {
                        while self.matches('\n') {}
                    } else {
                        break (Slash);
                    }
                }
                ' ' | '\r' | '\t' | '\n' => {}
                _ => return Some(Err(LexicalError::UnexpectedChar(c, row, col))),
            }
        };
        Some(Ok(Token {
            kind: TokenKind::Keyword(sym),
            row,
            col,
        }))
    }

    /// Gets the next character tracking row and col
    fn advance(&mut self) -> Option<(char, usize, usize)> {
        let (row, col) = (self.row, self.col);
        let c = self.source.next()?;
        self.col += 1;
        if c == '\n' {
            self.row += 1;
            self.col = 1;
        }
        Some((c, row, col))
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.source.peek() == Some(&expected) {
            self.source.next();
            true
        } else {
            false
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token<'a>, LexicalError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

impl<'a> Iterator for &'_ Scanner<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::tokens::{Keyword::*, TokenKind};
    use TokenKind::Keyword;

    use super::Scanner;

    #[test]
    fn tokenise_symbols() {
        let input = "{}(),.-+;*";
        let scanner = Scanner::new(input);
        let tokens: Vec<_> = scanner.map(|token| token.unwrap().kind).collect();

        let expected = [
            LeftBrace, RightBrace, LeftParen, RightParen, Comma, Dot, Minus, Plus, SemiColon, Star,
        ]
        .map(Keyword);

        assert_eq!(&expected[..], &tokens[..]);
    }
}
