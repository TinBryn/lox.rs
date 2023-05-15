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
        while self.peek()?.is_some() {
            let stmt = self.statement()?;
            statements.push(stmt);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> ParseResult<Stmt<'a>> {
        let stmt = if self.matches(var_match)?.is_some() {
            self.var_statement()?
        } else if self.matches(print_match)?.is_some() {
            self.print_statement()?
        } else {
            self.expression_statement()?
        };
        self.consume(TokenKind::Structure(Structure::SemiColon))?;
        Ok(stmt)
    }

    fn print_statement(&mut self) -> ParseResult<Stmt<'a>> {
        self.expression().map(Stmt::Print)
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt<'a>> {
        let expr = self.expression()?;
        Ok(Stmt::Expr(expr))
    }

    fn var_statement(&mut self) -> ParseResult<Stmt<'a>> {
        todo!()
    }

    fn expression(&mut self) -> ParseResult<Expr<'a>> {
        self.logical()
    }

    fn logical(&mut self) -> ParseResult<Expr<'a>> {
        let mut expr = self.equality()?;
        while let Some(op) = self.matches(logical_op)? {
            let right = self.equality()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Expr<'a>> {
        let mut expr = self.comparison()?;
        while let Some(op) = self.matches(eq_op)? {
            let right = self.comparison()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr<'a>> {
        let mut expr = self.term()?;
        while let Some(op) = self.matches(cmp_op)? {
            let right = self.term()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expr<'a>> {
        let mut expr = self.factor()?;
        while let Some(op) = self.matches(term_op)? {
            let right = self.factor()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expr<'a>> {
        let mut expr = self.unary()?;
        while let Some(op) = self.matches(factor_op)? {
            let right = self.unary()?;
            expr = Expr::from_binary(expr, op, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expr<'a>> {
        if let Some(op) = self.matches(unary_op)? {
            let expr = self.unary()?;
            Ok(Expr::from_unary(op, expr))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> ParseResult<Expr<'a>> {
        if let Some(token) = self.peek()? {
            match token.kind {
                TokenKind::Literal(lit) => {
                    self.advance()?;
                    match lit {
                        Literal::True => Ok(Expr::from_bool(true)),
                        Literal::False => Ok(Expr::from_bool(false)),
                        Literal::Nil => Ok(Expr::from_nil()),
                    }
                }
                TokenKind::Structure(st) => match st {
                    Structure::LeftParen => {
                        self.advance()?;
                        let expr = self.expression()?;
                        if !self.consume(TokenKind::Structure(Structure::RightParen))? {
                            Err(ParserError::BadStructure(None))
                        } else {
                            Ok(Expr::from_grouping(expr))
                        }
                    }
                    st => Err(ParserError::BadStructure(Some(st))),
                },
                TokenKind::Operator(op) => Err(ParserError::BadOperator(Some(op))),
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
                TokenKind::Keyword(_) => Err(ParserError::Unsupported),
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
            Err(ParserError::EndOfFileConsume)
        }
    }
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

fn logical_op(t: &TokenKind) -> Option<BinOp> {
    use Operator::*;
    match t {
        TokenKind::Operator(And) => Some(BinOp::And),
        TokenKind::Operator(Or) => Some(BinOp::Or),
        _ => None,
    }
}

fn eq_op(t: &TokenKind) -> Option<BinOp> {
    use Operator::*;
    match t {
        TokenKind::Operator(BangEqual) => Some(BinOp::Ne),
        TokenKind::Operator(EqualEqual) => Some(BinOp::Eq),
        _ => None,
    }
}

fn cmp_op(t: &TokenKind) -> Option<BinOp> {
    use Operator::*;
    if let TokenKind::Operator(op) = t {
        match op {
            Greater => Some(BinOp::Gt),
            GreaterEqual => Some(BinOp::Ge),
            Less => Some(BinOp::Lt),
            LessEqual => Some(BinOp::Le),
            _ => None,
        }
    } else {
        None
    }
}

fn term_op(t: &TokenKind) -> Option<BinOp> {
    use Operator::*;
    match t {
        TokenKind::Operator(op) => match op {
            Minus => Some(BinOp::Sub),
            Plus => Some(BinOp::Add),
            _ => None,
        },
        _ => None,
    }
}

fn factor_op(t: &TokenKind) -> Option<BinOp> {
    use Operator::*;
    match t {
        TokenKind::Operator(op) => match op {
            Slash => Some(BinOp::Div),
            Star => Some(BinOp::Mul),
            _ => None,
        },
        _ => None,
    }
}

fn unary_op(t: &TokenKind) -> Option<UnOp> {
    use Operator::*;
    match t {
        TokenKind::Operator(op) => match op {
            Minus => Some(UnOp::Neg),
            Bang => Some(UnOp::Not),
            _ => None,
        },

        _ => None,
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
