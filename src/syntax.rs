use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
}

#[derive(Debug, Copy, Clone)]
pub enum UnOp {
    Neg,
    Not,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Eq => f.write_str("=="),
            BinOp::Ne => f.write_str("!="),
            BinOp::Lt => f.write_char('<'),
            BinOp::Gt => f.write_char('>'),
            BinOp::Le => f.write_str("<="),
            BinOp::Ge => f.write_str(">="),
            BinOp::Add => f.write_char('+'),
            BinOp::Sub => f.write_char('-'),
            BinOp::Mul => f.write_char('*'),
            BinOp::Div => f.write_char('/'),
            BinOp::And => f.write_str("and"),
            BinOp::Or => f.write_str("or"),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOp::Neg => f.write_char('-'),
            UnOp::Not => f.write_char('!'),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary<'a> {
    pub left: Expr<'a>,
    pub operator: BinOp,
    pub right: Expr<'a>,
}

#[derive(Debug, Clone)]
pub struct Grouping<'a> {
    pub expression: Expr<'a>,
}

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    String(&'a str),
    Identifier(&'a str),
    Number(f64),
    True,
    False,
    Nil,
}

#[derive(Debug, Clone)]
pub struct Unary<'a> {
    pub operator: UnOp,
    pub expression: Expr<'a>,
}

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    Expr(Expr<'a>),
    Print(Expr<'a>),
    #[allow(dead_code)]
    Var(&'a str, Expr<'a>),
}

impl<'a> Stmt<'a> {
    pub fn display_lisp(&self) -> printer::Lisp<'a, '_> {
        printer::Lisp::new(self)
    }
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary(Box<Binary<'a>>),
    Grouping(Box<Grouping<'a>>),
    Literal(Literal<'a>),
    Unary(Box<Unary<'a>>),
}

impl<'a> Expr<'a> {
    pub fn from_binary(left: Self, operator: BinOp, right: Self) -> Self {
        Expr::Binary(Box::new(Binary {
            left,
            operator,
            right,
        }))
    }
    pub fn from_grouping(expression: Self) -> Self {
        Self::Grouping(Box::new(Grouping { expression }))
    }
    pub fn from_unary(operator: UnOp, expression: Self) -> Self {
        Self::Unary(Box::new(Unary {
            operator,
            expression,
        }))
    }
    pub fn from_number(n: f64) -> Self {
        Self::Literal(Literal::Number(n))
    }
    pub fn from_string(s: &'a str) -> Self {
        Self::Literal(Literal::String(s))
    }
    pub fn from_ident(id: &'a str) -> Self {
        Self::Literal(Literal::Identifier(id))
    }
    pub fn from_bool(b: bool) -> Self {
        Self::Literal(if b { Literal::True } else { Literal::False })
    }
    pub fn from_nil() -> Self {
        Self::Literal(Literal::Nil)
    }
}

pub mod visit;

mod printer;

#[cfg(test)]
mod test {
    use crate::syntax::Stmt;

    use super::{printer::Lisp, BinOp, Expr, UnOp};

    #[test]
    fn debug_expression_tree() {
        let e1 = Expr::from_unary(UnOp::Neg, Expr::from_number(123.));
        let e2 = Expr::from_grouping(Expr::from_number(45.67));
        let expr = Expr::from_binary(e1, BinOp::Mul, e2);
        let stmt = Stmt::Expr(expr);
        let s_expr = Lisp::new(&stmt);

        let expected = "(* (- 123) (group 45.67))";

        assert_eq!(format!("{s_expr}"), expected);
    }
}
