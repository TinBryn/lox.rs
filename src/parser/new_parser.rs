use crate::{
    error::{LexicalError, LoxParserError},
    scanner::Scanner,
    syntax::{BinOp, Expr, Stmt, UnOp},
    token::{Literal, Operator, Structure, Token, TokenKind},
};

pub struct LoxParser {
    tokens: Vec<Token>,
    current: usize,
}

impl LoxParser {
    #[allow(dead_code)]
    pub fn new(input: &str) -> Result<Self, LexicalError> {
        Ok(Self {
            tokens: Scanner::new(input).collect::<Result<_, _>>()?,
            current: 0,
        })
    }

    #[allow(dead_code)]
    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxParserError> {
        let expr = self.expression()?;

        Ok(vec![Stmt::Expr(expr)])
    }

    fn expression(&mut self) -> Result<Expr, LoxParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxParserError> {
        self.binary_expr(Self::comparison, |token| {
            Some(match token {
                TokenKind::Operator(Operator::EqualEqual) => BinOp::Eq,
                TokenKind::Operator(Operator::BangEqual) => BinOp::Ne,
                _ => return None,
            })
        })
    }

    fn comparison(&mut self) -> Result<Expr, LoxParserError> {
        self.binary_expr(Self::term, |token| {
            Some(match token {
                TokenKind::Operator(Operator::Greater) => BinOp::Gt,
                TokenKind::Operator(Operator::GreaterEqual) => BinOp::Ge,
                TokenKind::Operator(Operator::Less) => BinOp::Lt,
                TokenKind::Operator(Operator::LessEqual) => BinOp::Le,
                _ => return None,
            })
        })
    }

    fn term(&mut self) -> Result<Expr, LoxParserError> {
        self.binary_expr(Self::factor, |token| {
            Some(match token {
                TokenKind::Operator(Operator::Minus) => BinOp::Sub,
                TokenKind::Operator(Operator::Plus) => BinOp::Add,
                _ => return None,
            })
        })
    }

    fn factor(&mut self) -> Result<Expr, LoxParserError> {
        self.binary_expr(Self::unary, |token| {
            Some(match token {
                TokenKind::Operator(Operator::Slash) => BinOp::Div,
                TokenKind::Operator(Operator::Star) => BinOp::Mul,
                _ => return None,
            })
        })
    }

    fn unary(&mut self) -> Result<Expr, LoxParserError> {
        if let Some(token) = self.peek() {
            let op = match &token.kind {
                TokenKind::Operator(Operator::Bang) => UnOp::Not,
                TokenKind::Operator(Operator::Minus) => UnOp::Neg,
                _ => return self.primary(),
            };
            self.advance();
            let right = self.unary()?;
            Ok(Expr::from_unary(op, right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, LoxParserError> {
        if let Some(token) = self.tokens.get(self.current) {
            match &token.kind {
                TokenKind::Literal(Literal::True) => {
                    self.current += 1;
                    Ok(Expr::from_bool(true))
                }
                TokenKind::Literal(Literal::False) => {
                    self.current += 1;
                    Ok(Expr::from_bool(false))
                }
                TokenKind::Literal(Literal::Nil) => {
                    self.current += 1;
                    Ok(Expr::from_nil())
                }
                TokenKind::String(s) => {
                    self.current += 1;
                    Ok(Expr::from_string(s.clone()))
                }
                TokenKind::Number(n) => {
                    self.current += 1;
                    Ok(Expr::from_number(*n))
                }
                TokenKind::Structure(Structure::LeftParen) => {
                    self.current += 1;
                    let expr = self.expression()?;
                    match self.advance() {
                        Some(token) => match &token.kind {
                            TokenKind::Structure(Structure::RightParen) => {
                                Ok(Expr::from_grouping(expr))
                            }
                            _ => Err(LoxParserError::Message("Expected ')', but found something else")),
                        },
                        _ => Err(LoxParserError::Message("Tokens end without closing parenthesis")),
                    }
                }
                _ => Err(LoxParserError::Message("Invalid primary expression")),
            }
        } else {
            todo!()
        }
    }
    fn binary_expr<F, G>(&mut self, expr_fn: F, op_fn: G) -> Result<Expr, LoxParserError>
    where
        F: Fn(&mut LoxParser) -> Result<Expr, LoxParserError>,
        G: Fn(&TokenKind) -> Option<BinOp>,
    {
        let mut expr = expr_fn(self)?;
        while let Some(token) = self.peek() {
            let Some(op) = op_fn(&token.kind) else {
                break;
            };
            self.advance();
            let right = expr_fn(self)?;
            expr = Expr::from_binary(expr, op, right);
        }

        Ok(expr)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        let result = self.tokens.get(self.current);

        if result.is_some() {
            self.current += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::LoxParser;

    #[test]
    fn parse_number() {
        let input = "123.456";
        let expected = "123.456";

        let syntax = LoxParser::new(input)
            .expect("tokens")
            .parse()
            .expect("Successful parse of number");
        assert_eq!(expected, syntax[0].display_lisp().to_string());
    }

    #[test]
    fn parse_nested_expression() {
        let input = "true == (123 > 42 == -4 + 6 / (4 - 2))";
        let expected = "(== true (group (== (> 123 42) (+ (- 4) (/ 6 (group (- 4 2)))))))";

        let syntax = LoxParser::new(input)
            .expect("tokens")
            .parse()
            .expect("Parse nested expression");
        assert_eq!(expected, syntax[0].display_lisp().to_string());
    }

    #[test]
    fn parse_double_unary() {
        let input = "!-123";
        let expected = "(! (- 123))";

        let syntax = LoxParser::new(input).unwrap().parse().unwrap();
        assert_eq!(expected, syntax[0].display_lisp().to_string());
    }
}
