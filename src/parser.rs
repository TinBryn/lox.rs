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
            _ => self.expression(peek).map(Stmt::Expr)?,
        };
        self.consume(TokenKind::Structure(Structure::SemiColon))?;
        Ok(stmt)
    }

    fn print_statement(&mut self) -> ParseResult<Stmt<'a>> {
        let peek = self
            .advance()?
            .ok_or("print statement with nothing following")?;
        self.expression(peek).map(Stmt::Print)
    }

    fn var_statement(&mut self) -> ParseResult<Stmt<'a>> {
        todo!()
    }

    fn expression(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        self.unary(peek)
            .and_then(|expr| self.factor(expr))
            .and_then(|expr| self.term(expr))
            .and_then(|expr| self.comparison(expr))
            .and_then(|expr| self.equality(expr))
            .and_then(|expr| self.logical(expr))
    }

    fn logical(&mut self, mut expr: Expr<'a>) -> Result<Expr<'a>, ParserError> {
        while let Some(token) = self.peek()? {
            let op = match &token.kind {
                TokenKind::Operator(Operator::And) => BinOp::And,
                TokenKind::Operator(Operator::Or) => BinOp::Or,
                _ => break,
            };
            let peek = self
                .advance()?
                .ok_or("logical operator without right operand")?;
            let right = self
                .unary(peek)
                .and_then(|expr| self.factor(expr))
                .and_then(|expr| self.term(expr))
                .and_then(|expr| self.comparison(expr))
                .and_then(|expr| self.equality(expr))?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn equality(&mut self, mut left: Expr<'a>) -> Result<Expr<'a>, ParserError> {
        while let Some(token) = self.peek()? {
            let op = match &token.kind {
                TokenKind::Operator(Operator::BangEqual) => BinOp::Ne,
                TokenKind::Operator(Operator::EqualEqual) => BinOp::Eq,
                _ => break,
            };

            self.advance()?;
            let peek = self
                .advance()?
                .ok_or("equality operator without right operand")?;

            let right = self
                .unary(peek)
                .and_then(|expr| self.factor(expr))
                .and_then(|expr| self.term(expr))
                .and_then(|expr| self.comparison(expr))?;
            left = Expr::from_binary(left, op, right);
        }

        Ok(left)
    }

    fn comparison(&mut self, mut expr: Expr<'a>) -> Result<Expr<'a>, ParserError> {
        while let Some(token) = self.peek()? {
            use Operator::*;
            let op = match &token.kind {
                TokenKind::Operator(Greater) => BinOp::Gt,
                TokenKind::Operator(GreaterEqual) => BinOp::Ge,
                TokenKind::Operator(Less) => BinOp::Lt,
                TokenKind::Operator(LessEqual) => BinOp::Le,
                _ => break,
            };
            self.advance()?;
            let peek = self
                .advance()?
                .ok_or("comparison operator without right operand")?;
            let right = self
                .unary(peek)
                .and_then(|expr| self.factor(expr))
                .and_then(|expr| self.term(expr))?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn term(&mut self, mut expr: Expr<'a>) -> Result<Expr<'a>, ParserError> {
        while let Some(token) = self.peek()? {
            use Operator::*;
            let op = match &token.kind {
                TokenKind::Operator(Minus) => BinOp::Sub,
                TokenKind::Operator(Plus) => BinOp::Add,
                _ => break,
            };
            self.advance()?;

            let peek = self
                .advance()?
                .ok_or("term operator without right operator")?;
            let right = self.unary(peek).and_then(|expr| self.factor(expr))?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn factor(&mut self, mut expr: Expr<'a>) -> Result<Expr<'a>, ParserError> {
        use Operator::*;
        while let Some(token) = self.peek()? {
            let op = match &token.kind {
                TokenKind::Operator(Slash) => BinOp::Div,
                TokenKind::Operator(Star) => BinOp::Mul,
                _ => break,
            };
            self.advance()?;
            let peek = self
                .advance()?
                .ok_or("factor operator without right operand")?;
            let right = self.unary(peek)?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn unary(&mut self, peek: Token<'a>) -> ParseResult<Expr<'a>> {
        let op = match &peek.kind {
            TokenKind::Operator(Operator::Minus) => UnOp::Neg,
            TokenKind::Operator(Operator::Bang) => UnOp::Not,
            _ => return self.primary(peek),
        };
        let peek = self.advance()?.ok_or("unary operator without operand")?;
        let expr = self.unary(peek)?;
        Ok(Expr::from_unary(op, expr))
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
                    let peek = self
                        .advance()?
                        .ok_or("parenthesis without following expression")?;
                    let expr = self.expression(peek)?;
                    if !self.consume(TokenKind::Structure(Structure::RightParen))? {
                        Err("no terminating parenthesis")?
                    } else {
                        Ok(Expr::from_grouping(expr))
                    }
                }
                st => Err(ParserError::BadStructure(Some(st))),
            },
            TokenKind::Number(n) => Ok(Expr::from_number(n)),
            TokenKind::String(s) => Ok(Expr::from_string(s)),
            TokenKind::Identifier(id) => Ok(Expr::from_ident(id)),
            TokenKind::Operator(op) => Err(ParserError::BadOperator(Some(op))),
            TokenKind::Keyword(_) => Err("This keyword is not yet supported")?,
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
            Err("Tried to consume token, but end of file")?
        }
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
