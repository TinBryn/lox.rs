use crate::error::LexicalError;

use super::tokens::{Keyword, Token, TokenKind};

pub struct Scanner<'a> {
    source: &'a str,
    current: usize,
    start: usize,
    row: usize,
    col: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            row: 1,
            col: 1,
        }
    }

    fn scan_token(&mut self) -> Option<Result<Token<'a>, LexicalError>> {
        use Keyword::*;
        self.start = self.current;
        let (mut c, row, col) = self.advance()?;
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
                        while self.look_ahead()? != '\n' {
                            self.advance();
                        }
                        c = self.restart()?;
                    } else {
                        break (Slash);
                    }
                }
                ' ' | '\r' | '\t' | '\n' => {
                    c = self.restart()?;
                }
                '"' => return Some(self.string(row, col)),
                '0'..='9' => return Some(self.number(row, col)),
                _ => return Some(Err(LexicalError::UnexpectedChar(c, row, col))),
            }
        };
        Some(Ok(Token {
            kind: TokenKind::Keyword(sym),
            row,
            col,
        }))
    }

    fn restart(&mut self) -> Option<char> {
        self.start = self.current;
        let mut iter = self.current().chars();
        let pre_len = iter.as_str().len();
        let c = iter.next()?;
        let post_len = iter.as_str().len();

        self.current += pre_len - post_len;

        self.col += 1;
        if c == '\n' {
            self.row += 1;
            self.col = 1;
        }
        Some(c)
    }

    fn current(&self) -> &'a str {
        &self.source[self.current..]
    }

    /// Gets the next character tracking row and col
    fn advance(&mut self) -> Option<(char, usize, usize)> {
        let (row, col) = (self.row, self.col);
        let mut iter = self.current().chars();
        let pre_len = iter.as_str().len();
        let c = iter.next()?;
        let post_len = iter.as_str().len();

        self.current += pre_len - post_len;

        self.col += 1;
        if c == '\n' {
            self.row += 1;
            self.col = 1;
        }
        Some((c, row, col))
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.current().starts_with(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn look_ahead(&mut self) -> Option<char> {
        self.current().chars().next()
    }

    fn look_ahead_nth(&mut self, n: usize) -> Option<char> {
        self.current().chars().nth(n)
    }

    fn string(&mut self, row: usize, col: usize) -> Result<Token<'a>, LexicalError> {
        self.start = self.current;
        while self
            .look_ahead()
            .ok_or(LexicalError::UnterminatedString(row, col))?
            != '"'
        {
            let (_, _, _) = self
                .advance()
                .ok_or(LexicalError::UnterminatedString(row, col))?;
        }
        let s = self.sub_str();
        let s = TokenKind::String(s);
        let token = Token { kind: s, row, col };

        self.advance();

        Ok(token)
    }

    fn sub_str(&mut self) -> &'a str {
        &self.source[self.start..self.current]
    }

    fn number(&mut self, row: usize, col: usize) -> Result<Token<'a>, LexicalError> {
        while matches!(self.look_ahead(), Some('0'..='9')) {
            self.advance();
        }

        if self.look_ahead() == Some('.') && matches!(self.look_ahead_nth(1), Some('0'..='9')) {
            self.advance();

            while matches!(self.look_ahead(), Some('0'..='9')) {
                self.advance();
            }
        }

        self.sub_str()
            .parse()
            .map_err(|_| LexicalError::ParseNumberError(row, col))
            .map(|n| Token {
                kind: TokenKind::Number(n),
                row,
                col,
            })
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token<'a>, LexicalError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        error::LexicalError,
        interpreter::tokens::{Keyword::*, TokenKind},
    };
    use TokenKind::{Keyword, Number, String};

    use super::Scanner;

    #[test]
    fn tokenise_symbols() {
        let input = "{ } ( ) , . - + ; *";
        let scanner = Scanner::new(input);
        let tokens: Vec<_> = scanner.map(|token| token.unwrap().kind).collect();

        let expected = [
            LeftBrace, RightBrace, LeftParen, RightParen, Comma, Dot, Minus, Plus, SemiColon, Star,
        ]
        .map(Keyword);

        assert_eq!(&expected[..], &tokens[..]);
    }

    #[test]
    fn tokenise_compound_symbols() {
        let scanner = Scanner::new("== != <= >=");

        let tokens: Vec<_> = scanner.map(|token| token.unwrap().kind).collect();

        let expected = [EqualEqual, BangEqual, LessEqual, GreaterEqual].map(Keyword);

        assert_eq!(&expected[..], &tokens[..]);
    }

    #[test]
    fn tokenise_ignoring_comments() {
        let input = r#"
        // this is a comment
        ({ })
        "#;

        let scanner = Scanner::new(input);

        let tokens: Vec<_> = scanner.map(|token| token.unwrap().kind).collect();

        let expected = [LeftParen, LeftBrace, RightBrace, RightParen].map(Keyword);

        assert_eq!(&expected[..], &tokens[..]);
    }

    #[test]
    fn parse_strings() {
        let scanner = Scanner::new("\"hello\" \"world\"");

        let tokens: Vec<_> = scanner.map(|token| token.unwrap().kind).collect();

        let expected = ["hello", "world"].map(String);

        assert_eq!(&expected[..], &tokens[..]);
    }

    #[test]
    fn parse_unterminated_string() {
        let scanner = Scanner::new("\"hello\" \"world");

        let tokens: Vec<_> = scanner.map(|r| r.map(|t| t.kind)).collect();

        let expected = [
            Ok(String("hello")),
            Err(LexicalError::UnterminatedString(1, 8)),
        ];

        assert_eq!(&expected[..], &tokens[..]);
    }

    #[test]
    fn parse_int() {
        let scanner = Scanner::new("123");
        let tokens: Vec<_> = scanner.map(|t| t.unwrap().kind).collect();

        let expected = [Number(123.)];

        assert_eq!(tokens[..], expected[..])
    }

    #[test]
    fn parse_float() {
        let scanner = Scanner::new("420.69");
        let tokens: Vec<_> = scanner.map(|t| t.unwrap().kind).collect();

        let expected = [Number(420.69)];

        assert_eq!(tokens[..], expected[..])
    }
}
