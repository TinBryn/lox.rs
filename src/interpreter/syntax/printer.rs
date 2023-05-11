use std::fmt::{self, Display, Formatter, Write};

use super::{visit::Visitor, Binary, Expr, Grouping, Literal, Unary};

pub struct ASTPrinter<'a, 'b> {
    f: &'a mut Formatter<'b>,
}

impl<'b> Visitor<fmt::Result> for ASTPrinter<'_, 'b> {
    fn visit_binary(&mut self, binary: &Binary) -> fmt::Result {
        self.f.write_char('(')?;
        Display::fmt(&binary.operator, self.f)?;
        self.f.write_char(' ')?;
        binary.left.accept(&mut *self)?;
        self.f.write_char(' ')?;
        binary.right.accept(&mut *self)?;
        self.f.write_char(')')
    }

    fn visit_group(&mut self, group: &Grouping) -> fmt::Result {
        self.f.write_char('(')?;
        self.f.write_str("group ")?;
        group.expression.accept(&mut *self)?;
        self.f.write_char(')')
    }

    fn visit_literal(&mut self, lit: &Literal) -> fmt::Result {
        match lit {
            Literal::String(str) => self.f.write_fmt(format_args!("{str:?}")),
            Literal::Identifier(id) => self.f.write_fmt(format_args!("`{id}`")),
            Literal::Number(n) => Display::fmt(n, self.f),
            Literal::True => self.f.write_str("true"),
            Literal::False => self.f.write_str("false"),
            Literal::Nil => self.f.write_str("nil"),
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> fmt::Result {
        self.f.write_char('(')?;
        Display::fmt(&unary.operator, self.f)?;
        self.f.write_char(' ')?;
        unary.expression.accept(&mut *self)?;
        self.f.write_char(')')
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SExpr<'a, 'b>(&'b Expr<'a>);

impl<'a, 'b> SExpr<'a, 'b> {
    pub fn new(expr: &'b Expr<'a>) -> Self {
        Self(expr)
    }
}

impl<'a, 'b> Display for SExpr<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.accept(&mut ASTPrinter { f })
    }
}
