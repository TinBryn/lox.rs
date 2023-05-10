use crate::error::LexicalError;

use super::tokens::{Keyword, Token, TokenKind};

#[derive(Debug, Clone, Copy)]
struct Pos {
    index: usize,
    row: usize,
    col: usize,
}

pub struct Scanner<'a> {
    source: &'a str,
    start: Pos,
    current: Pos,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: Pos {
                index: 0,
                row: 1,
                col: 1,
            },
            current: Pos {
                index: 0,
                row: 1,
                col: 1,
            },
        }
    }

    fn scan_token(&mut self) -> Option<Result<Token<'a>, LexicalError>> {
        use Keyword::*;
        self.start = self.current;
        let mut c = self.advance()?;
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
                '"' => return Some(self.string()),
                '0'..='9' => return Some(self.number()),
                'a'..='z' | 'A'..='Z' | '_' => return Some(self.identifier()),
                _ => {
                    return Some(Err(LexicalError::UnexpectedChar(
                        c,
                        self.current.row,
                        self.current.col,
                    )))
                }
            }
        };
        Some(Ok(Token {
            kind: TokenKind::Keyword(sym),
            row: self.start.row,
            col: self.start.col,
        }))
    }

    fn restart(&mut self) -> Option<char> {
        let mut iter = self.rest().chars();
        let pre_len = iter.as_str().len();
        let c = iter.next()?;
        let post_len = iter.as_str().len();

        self.start = self.current;
        self.current.index += pre_len - post_len;

        if c == '\n' {
            self.current.row += 1;
            self.current.col = 1;
        } else {
            self.current.col += 1;
        }
        Some(c)
    }

    fn rest(&self) -> &'a str {
        &self.source[self.current.index..]
    }

    /// Gets the next character tracking row and col
    fn advance(&mut self) -> Option<char> {
        let mut iter = self.rest().chars();
        let pre_len = iter.as_str().len();
        let c = iter.next()?;
        let post_len = iter.as_str().len();

        self.current.index += pre_len - post_len;

        self.current.col += 1;
        if c == '\n' {
            self.current.row += 1;
            self.current.col = 1;
        }
        Some(c)
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.rest().starts_with(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn look_ahead(&mut self) -> Option<char> {
        self.rest().chars().next()
    }

    fn look_ahead_nth(&mut self, n: usize) -> Option<char> {
        self.rest().chars().nth(n)
    }

    fn string(&mut self) -> Result<Token<'a>, LexicalError> {
        while self.look_ahead().ok_or(LexicalError::UnterminatedString(
            self.start.row,
            self.start.col,
        ))? != '"'
        {
            self.advance().ok_or(LexicalError::UnterminatedString(
                self.start.row,
                self.start.col,
            ))?;
        }
        self.advance();

        let s = self.sub_str();
        let token = Token {
            kind: TokenKind::String(&s[1..s.len() - 1]),
            row: self.start.row,
            col: self.start.col,
        };
        Ok(token)
    }

    fn sub_str(&mut self) -> &'a str {
        &self.source[self.start.index..self.current.index]
    }

    fn number(&mut self) -> Result<Token<'a>, LexicalError> {
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
            .map_err(|_| LexicalError::ParseNumberError(self.start.row, self.start.col))
            .map(|n| Token {
                kind: TokenKind::Number(n),
                row: self.start.row,
                col: self.start.col,
            })
    }

    fn identifier(&mut self) -> Result<Token<'a>, LexicalError> {
        while let Some(c) = self.look_ahead() {
            if !c.is_alphanumeric() {
                break;
            }
            self.advance();
        }

        let token = self.sub_str();
        use Keyword::*;
        let token = match token {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "fun" => Fun,
            "for" => For,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            _ => {
                let token = TokenKind::Identifier(token);

                let token = Token {
                    kind: token,
                    row: self.start.row,
                    col: self.start.col,
                };

                return Ok(token);
            }
        };

        let token = TokenKind::Keyword(token);

        let token = Token {
            kind: token,
            row: self.start.row,
            col: self.start.col,
        };

        Ok(token)
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
            Err(LexicalError::UnterminatedString(1, 9)),
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

    #[test]
    fn lex_some_complex_code() {
        let input = r#"
fun hello(name) {
    print "hello ";
    print name;
}
        "#;
        let tokens: Vec<_> = Scanner::new(input).collect::<Result<_, _>>().unwrap();

        for token in tokens {
            println!("{token:?}")
        }
    }
}
