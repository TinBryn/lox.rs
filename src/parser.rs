use crate::{
    syntax::Stmt,
    token::{Keyword, Literal},
};

use super::{
    error::{LexicalError, ParserError},
    scanner::Scanner,
    syntax::{BinOp, Expr, UnOp},
    token::{Operator, Structure, Token, TokenKind},
};

pub struct Parser<'a> {
    tokens: Scanner<'a>,
    peeked: Option<Option<Result<Token<'a>, LexicalError>>>,
}

pub type ParseResult<T> = Result<T, ParserError>;

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: Scanner::new(input),
            peeked: None,
        }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Stmt<'a>>> {
        let mut statements = Vec::new();
        while let Some(peek) = self.advance()? {
            let stmt = self.statement(peek)?;
            statements.push(stmt);
        }

        Ok(statements)
    }

    fn statement(&mut self, peek: Token<'a>) -> ParseResult<Stmt<'a>> {
        let stmt = match &peek.kind {
            TokenKind::Keyword(Keyword::Var) => self.var_statement()?,
            TokenKind::Keyword(Keyword::Print) => self.print_statement()?,
            _ => self.expression_statement(peek)?,
        };
        self.consume(TokenKind::Structure(Structure::SemiColon))?;
        Ok(stmt)
    }

    fn print_statement(&mut self) -> ParseResult<Stmt<'a>> {
        let peek = self.advance()?.unwrap();
        self.expression(peek).map(Stmt::Print)
    }

    fn expression_statement(&mut self, peek: Token<'a>) -> ParseResult<Stmt<'a>> {
        let expr = self.expression(peek)?;
        Ok(Stmt::Expr(expr))
    }

    fn var_statement(&mut self) -> ParseResult<Stmt<'a>> {
        todo!()
    }

    fn expression(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        self.logical(peek)
    }

    fn logical(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        let mut expr = self.equality(peek)?;
        while let Some(token) = self.peek()? {
            let op = match &token.kind {
                TokenKind::Operator(Operator::And) => BinOp::And,
                TokenKind::Operator(Operator::Or) => BinOp::Or,
                _ => break,
            };
            let peek = self.advance()?.unwrap();
            let right = self.equality(peek)?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn equality(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        let mut expr = self.comparison(peek)?;
        while let Some(token) = self.peek()? {
            let op = match &token.kind {
                TokenKind::Operator(Operator::BangEqual) => BinOp::Ne,
                TokenKind::Operator(Operator::EqualEqual) => BinOp::Eq,
                _ => break,
            };

            self.advance()?;
            let peek = self.advance()?.unwrap();
            let right = self.comparison(peek)?;
            expr = Expr::from_binary(expr, op, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        let mut expr = self.term(peek)?;
        while let Some(token) = self.peek()? {
            let Some(op) = comparison_op(&token.kind) else {
                break;
            };
            self.advance()?;
            let peek = self.advance()?.unwrap();
            let right = self.term(peek)?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn term(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        use Operator::*;
        let mut expr = self.factor(peek)?;
        loop {
            if let Some(token) = self.peek()? {
                expr = match &token.kind {
                    TokenKind::Operator(Minus) => self.term_right(expr, BinOp::Sub)?,
                    TokenKind::Operator(Plus) => self.term_right(expr, BinOp::Add)?,
                    _ => return Ok(expr),
                }
            } else {
                return Ok(expr);
            }
        }
    }

    fn term_right(&mut self, expr: Expr<'a>, op: BinOp) -> Result<Expr<'a>, ParserError> {
        self.advance()?;
        let peek = self.advance()?.unwrap();
        let right = self.factor(peek)?;
        let e = Expr::from_binary(expr, op, right);
        Ok(e)
    }

    fn factor(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        use Operator::*;
        let mut expr = self.unary(peek)?;
        loop {
            if let Some(token) = self.peek()? {
                expr = match &token.kind {
                    TokenKind::Operator(Slash) => self.factor_right(expr, BinOp::Div)?,
                    TokenKind::Operator(Star) => self.factor_right(expr, BinOp::Mul)?,
                    _ => return Ok(expr),
                }
            } else {
                return Ok(expr);
            }
        }
    }

    fn factor_right(&mut self, expr: Expr<'a>, op: BinOp) -> Result<Expr<'a>, ParserError> {
        self.advance()?;
        let peek = self.advance()?.unwrap();
        let right = self.unary(peek)?;
        let expr = Expr::from_binary(expr, op, right);
        Ok(expr)
    }

    fn unary(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        match &peek.kind {
            TokenKind::Operator(op) => match op {
                Operator::Minus => {
                    let peek = self.advance()?.unwrap();
                    let expr = self.unary(peek)?;
                    Ok(Expr::from_unary(UnOp::Neg, expr))
                }
                Operator::Bang => {
                    let peek = self.advance()?.unwrap();
                    let expr = self.unary(peek)?;
                    Ok(Expr::from_unary(UnOp::Not, expr))
                }
                _ => self.primary(peek),
            },
            _ => self.primary(peek),
        }
    }

    fn primary(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        match peek.kind {
            TokenKind::Literal(lit) => match lit {
                Literal::True => Ok(Expr::from_bool(true)),
                Literal::False => Ok(Expr::from_bool(false)),
                Literal::Nil => Ok(Expr::from_nil()),
            },
            TokenKind::Structure(st) => match st {
                Structure::LeftParen => {
                    let peek = self.advance()?.unwrap();
                    let expr = self.expression(peek)?;
                    if !self.consume(TokenKind::Structure(Structure::RightParen))? {
                        Err(ParserError::BadStructure(None))
                    } else {
                        Ok(Expr::from_grouping(expr))
                    }
                }
                st => Err(ParserError::BadStructure(Some(st))),
            },
            TokenKind::Operator(op) => Err(ParserError::BadOperator(Some(op))),
            TokenKind::Number(n) => Ok(Expr::from_number(n)),
            TokenKind::String(s) => Ok(Expr::from_string(s)),
            TokenKind::Identifier(id) => Ok(Expr::from_ident(id)),
            TokenKind::Keyword(_) => Err(ParserError::Unsupported),
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
            Err(ParserError::EndOfFileConsume)
        }
    }
}

fn comparison_op(kind: &TokenKind) -> Option<BinOp> {
    use Operator::*;
    Some(match kind {
        TokenKind::Operator(Greater) => BinOp::Gt,
        TokenKind::Operator(GreaterEqual) => BinOp::Ge,
        TokenKind::Operator(Less) => BinOp::Lt,
        TokenKind::Operator(LessEqual) => BinOp::Le,

        _ => return None,
    })
}

fn print_match(t: &TokenKind) -> Option<()> {
    if let TokenKind::Keyword(Keyword::Print) = t {
        Some(())
    } else {
        None
    }
}

fn var_match(t: &TokenKind) -> Option<()> {
    if let TokenKind::Keyword(Keyword::Var) = t {
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::Parser;

    #[test]
    fn parse_number() {
        let input = "123.456;";
        let expected = "123.456";

        let syntax = Parser::new(input).parse().unwrap();
        assert_eq!(expected, syntax[0].display_lisp().to_string());
    }

    #[test]
    fn parse_nested_expression() {
        let input = "true == (123 > 42 == -4 + 6 / (4 - 2));";
        let expected = "(== true (group (== (> 123 42) (+ (- 4) (/ 6 (group (- 4 2)))))))";

        let syntax = Parser::new(input).parse().unwrap();
        assert_eq!(expected, syntax[0].display_lisp().to_string());
    }

    #[test]
    fn parse_double_unary() {
        let input = "!-123;";
        let expected = "(! (- 123))";

        let syntax = Parser::new(input).parse().unwrap();
        assert_eq!(expected, syntax[0].display_lisp().to_string());
    }
}
