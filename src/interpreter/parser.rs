use crate::{
    error::{LexicalError, ParserError},
    interpreter::tokens::{LangToken, Operator},
};

use super::{
    scanner::Scanner,
    syntax::Expr,
    tokens::{Token, TokenKind},
};

pub struct Parser<'a> {
    tokens: Scanner<'a>,
    peeked: Option<Option<Result<Token<'a>, LexicalError>>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: Scanner::new(input),
            peeked: None,
        }
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr<'a>, ParserError> {
        let mut expr = self.comparison()?;
        while let Some(op) = self.matches(eq_op)? {
            let right = self.comparison()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr<'a>, ParserError> {
        let mut expr = self.term()?;
        while let Some(op) = self.matches(cmp_op)? {
            let right = self.term()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr<'a>, ParserError> {
        let mut expr = self.factor()?;
        while let Some(op) = self.matches(term_op)? {
            let right = self.factor()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr<'a>, ParserError> {
        let mut expr = self.unary()?;
        while let Some(op) = self.matches(factor_op)? {
            let right = self.unary()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr<'a>, ParserError> {
        todo!()
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

    fn peek(&mut self) -> Result<Option<&Token>, LexicalError> {
        self.peeked
            .get_or_insert_with(|| self.tokens.next())
            .as_ref()
            .map(|r| r.as_ref())
            .transpose()
            .map_err(|e| *e)
    }

    fn advance(&mut self) -> Result<Option<Token>, LexicalError> {
        self.peeked
            .take()
            .unwrap_or_else(|| self.tokens.next())
            .transpose()
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
