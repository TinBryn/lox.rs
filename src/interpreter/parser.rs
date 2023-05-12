use crate::{
    error::{LexicalError, ParserError},
    interpreter::tokens::{LangToken, Operator},
};

use super::{
    scanner::Scanner,
    syntax::Expr,
    tokens::{Keyword, Structure, Token, TokenKind},
};

pub struct Parser<'a> {
    tokens: Scanner<'a>,
    peeked: Option<Option<Result<Token<'a>, LexicalError>>>,
}

pub type ParseResult<'a> = Result<Expr<'a>, ParserError>;

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: Scanner::new(input),
            peeked: None,
        }
    }

    pub fn parse(&mut self) -> ParseResult<'a> {
        self.expression()
    }

    fn expression(&mut self) -> ParseResult<'a> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<'a> {
        let mut expr = self.comparison()?;
        while let Some(op) = self.matches(eq_op)? {
            let right = self.comparison()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<'a> {
        let mut expr = self.term()?;
        while let Some(op) = self.matches(cmp_op)? {
            let right = self.term()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<'a> {
        let mut expr = self.factor()?;
        while let Some(op) = self.matches(term_op)? {
            let right = self.factor()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<'a> {
        let mut expr = self.unary()?;
        while let Some(op) = self.matches(factor_op)? {
            let right = self.unary()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<'a> {
        if let Some(op) = self.matches(unary_op)? {
            let expr = self.unary()?;
            Ok(Expr::from_unary(op, expr))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> ParseResult<'a> {
        if let Some(token) = self.peek()? {
            match token.kind {
                TokenKind::LangToken(LangToken::Keyword(kw)) => match kw {
                    Keyword::True => {
                        self.advance()?;
                        Ok(Expr::from_bool(true))
                    }
                    Keyword::False => {
                        self.advance()?;
                        Ok(Expr::from_bool(false))
                    }
                    Keyword::Nil => {
                        self.advance()?;
                        Ok(Expr::from_nil())
                    }
                    _kw => Err(ParserError::Unsupported),
                },
                TokenKind::LangToken(LangToken::Structure(st)) => match st {
                    Structure::LeftParen => {
                        self.advance()?;
                        let expr = self.expression()?;
                        if !self.consume(TokenKind::LangToken(LangToken::Structure(
                            Structure::RightParen,
                        )))? {
                            Err(ParserError::BadStructure(None))
                        } else {
                            Ok(Expr::from_grouping(expr))
                        }
                    }
                    st => Err(ParserError::BadStructure(Some(st))),
                },
                TokenKind::LangToken(LangToken::Operator(op)) => {
                    Err(ParserError::BadOperator(Some(op)))
                }
                TokenKind::Number(n) => {
                    self.advance()?;
                    Ok(Expr::from_number(n))
                }
                TokenKind::String(s) => {
                    self.advance()?;
                    Ok(Expr::from_string(s))
                }
                TokenKind::Identifier(id) => {
                    self.advance()?;
                    Ok(Expr::from_ident(id))
                }
            }
        } else {
            Err(ParserError::EndOfFile)
        }
    }

    fn matches<T, P: FnOnce(&TokenKind) -> Option<T>>(
        &mut self,
        p: P,
    ) -> Result<Option<T>, ParserError> {
        if let Some(t) = self.peek()?.and_then(|token| p(&token.kind)) {
            self.advance()?;
            Ok(Some(t))
        } else {
            Ok(None)
        }
    }

    fn peek(&mut self) -> Result<Option<&Token<'a>>, LexicalError> {
        self.peeked
            .get_or_insert_with(|| self.tokens.next())
            .as_ref()
            .map(|r| r.as_ref())
            .transpose()
            .map_err(|e| *e)
    }

    fn advance(&mut self) -> Result<Option<Token<'a>>, LexicalError> {
        self.peeked
            .take()
            .unwrap_or_else(|| self.tokens.next())
            .transpose()
    }

    fn consume(&mut self, token_kind: TokenKind) -> Result<bool, ParserError> {
        if let Some(token) = self.advance()? {
            Ok(token_kind == token.kind)
        } else {
            Err(ParserError::EndOfFile)
        }
    }
}

fn eq_op(t: &TokenKind) -> Option<Operator> {
    use Operator::*;
    match t {
        TokenKind::LangToken(LangToken::Operator(t @ (BangEqual | EqualEqual))) => Some(*t),
        _ => None,
    }
}

fn cmp_op(t: &TokenKind) -> Option<Operator> {
    use Operator::*;
    match t {
        TokenKind::LangToken(LangToken::Operator(
            t @ (Greater | GreaterEqual | Less | LessEqual),
        )) => Some(*t),
        _ => None,
    }
}

fn term_op(t: &TokenKind) -> Option<Operator> {
    use Operator::*;
    match t {
        TokenKind::LangToken(LangToken::Operator(t @ (Minus | Plus))) => Some(*t),
        _ => None,
    }
}

fn factor_op(t: &TokenKind) -> Option<Operator> {
    use Operator::*;
    match t {
        TokenKind::LangToken(LangToken::Operator(t @ (Slash | Star))) => Some(*t),
        _ => None,
    }
}

fn unary_op(t: &TokenKind) -> Option<Operator> {
    use Operator::*;
    match t {
        TokenKind::LangToken(LangToken::Operator(t @ (Bang | Minus))) => Some(*t),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::Parser;

    #[test]
    fn parse_number() {
        let input = "123.456";
        let expected = "123.456";

        let syntax = Parser::new(input).parse().unwrap();
        assert_eq!(expected, syntax.as_ast().to_string());
    }

    #[test]
    fn parse_nested_expression() {
        let input = "true == (123 > 42 == -4 + 6 / (4 - 2))";
        let expected = "(== true (group (== (> 123 42) (+ (- 4) (/ 6 (group (- 4 2)))))))";

        let syntax = Parser::new(input).parse().unwrap();
        assert_eq!(expected, syntax.as_ast().to_string());
    }

    #[test]
    fn parse_double_unary() {
        let input = "!-123";
        let expected = "(! (- 123))";

        let syntax = Parser::new(input).parse().unwrap();
        assert_eq!(expected, syntax.as_ast().to_string());
    }
}
