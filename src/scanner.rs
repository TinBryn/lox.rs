use crate::{
    error::LexicalError,
    tokens::{Token, TokenKind, TokenMeta},
};

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
        use super::tokens::Operator::*;
        use super::tokens::Structure::*;

        self.start = self.current;
        let mut c = self.advance()?;
        let sym = loop {
            match c {
                '(' => break TokenKind::Structure(LeftParen),
                ')' => break TokenKind::Structure(RightParen),
                '{' => break TokenKind::Structure(LeftBrace),
                '}' => break TokenKind::Structure(RightBrace),
                ',' => break TokenKind::Structure(Comma),
                '.' => break TokenKind::Structure(Dot),
                ';' => break TokenKind::Structure(SemiColon),
                '-' => break TokenKind::Operator(Minus),
                '+' => break TokenKind::Operator(Plus),
                '*' => break TokenKind::Operator(Star),
                '!' => {
                    break if self.matches('=') {
                        TokenKind::Operator(BangEqual)
                    } else {
                        TokenKind::Operator(Bang)
                    }
                }
                '=' => {
                    break if self.matches('=') {
                        TokenKind::Operator(EqualEqual)
                    } else {
                        TokenKind::Operator(Equal)
                    }
                }
                '<' => {
                    break if self.matches('=') {
                        TokenKind::Operator(LessEqual)
                    } else {
                        TokenKind::Operator(Less)
                    }
                }
                '>' => {
                    break if self.matches('=') {
                        TokenKind::Operator(GreaterEqual)
                    } else {
                        TokenKind::Operator(Greater)
                    }
                }
                '/' => {
                    if self.matches('/') {
                        while self.look_ahead()? != '\n' {
                            self.advance();
                        }
                        c = self.restart()?;
                    } else {
                        break TokenKind::Operator(Slash);
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
            kind: sym,
            meta: TokenMeta {
                row: self.start.row,
                col: self.start.col,
            },
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
            meta: TokenMeta {
                row: self.start.row,
                col: self.start.col,
            },
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
                meta: TokenMeta {
                    row: self.start.row,
                    col: self.start.col,
                },
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
        use super::tokens::Keyword::*;
        use super::tokens::Operator::*;

        let token = match token {
            "and" => TokenKind::Operator(And),
            "or" => TokenKind::Operator(Or),
            "class" => TokenKind::Keyword(Class),
            "else" => TokenKind::Keyword(Else),
            "false" => TokenKind::Keyword(False),
            "fun" => TokenKind::Keyword(Fun),
            "for" => TokenKind::Keyword(For),
            "if" => TokenKind::Keyword(If),
            "nil" => TokenKind::Keyword(Nil),
            "print" => TokenKind::Keyword(Print),
            "return" => TokenKind::Keyword(Return),
            "super" => TokenKind::Keyword(Super),
            "this" => TokenKind::Keyword(This),
            "true" => TokenKind::Keyword(True),
            "var" => TokenKind::Keyword(Var),
            "while" => TokenKind::Keyword(While),
            _ => {
                let token = TokenKind::Identifier(token);

                let token = Token {
                    kind: token,
                    meta: TokenMeta {
                        row: self.start.row,
                        col: self.start.col,
                    },
                };

                return Ok(token);
            }
        };

        let token = Token {
            kind: token,
            meta: TokenMeta {
                row: self.start.row,
                col: self.start.col,
            },
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
        tokens::{Operator::*, Structure::*, TokenKind},
    };
    use TokenKind::{Number, String};
    // use TokenKind::LangToken;

    use super::Scanner;

    #[test]
    fn tokenise_symbols() {
        let input = "{ } ( ) , . - + ; *";
        let scanner = Scanner::new(input);
        let tokens: Vec<_> = scanner.map(|token| token.unwrap().kind).collect();

        let expected = [
            TokenKind::Structure(LeftBrace),
            TokenKind::Structure(RightBrace),
            TokenKind::Structure(LeftParen),
            TokenKind::Structure(RightParen),
            TokenKind::Structure(Comma),
            TokenKind::Structure(Dot),
            TokenKind::Operator(Minus),
            TokenKind::Operator(Plus),
            TokenKind::Structure(SemiColon),
            TokenKind::Operator(Star),
        ];

        assert_eq!(&expected[..], &tokens[..]);
    }

    #[test]
    fn tokenise_compound_symbols() {
        let scanner = Scanner::new("== != <= >=");

        let tokens: Vec<_> = scanner.map(|token| token.unwrap().kind).collect();

        let expected = [EqualEqual, BangEqual, LessEqual, GreaterEqual].map(TokenKind::Operator);

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

        let expected = [LeftParen, LeftBrace, RightBrace, RightParen].map(TokenKind::Structure);

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
    var cond = true;
    if (cond) {
        print 123 + 456;
    }
}
"#
        .trim();
        let tokens: Vec<_> = Scanner::new(input).collect::<Result<_, _>>().unwrap();

        for token in tokens {
            println!("{token:?}")
        }
    }
}
