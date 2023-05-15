use std::fmt::{self, Display, Formatter, Write};

use super::{
    visit::{ExprVisitor, StmtVisitor},
    Binary, Expr, Grouping, Literal, Stmt, Unary,
};

pub struct LispAstPrinter<'a, 'b> {
    f: &'a mut Formatter<'b>,
}

impl<'b> ExprVisitor<fmt::Result> for LispAstPrinter<'_, 'b> {
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

impl<'b> StmtVisitor<fmt::Result> for LispAstPrinter<'_, 'b> {
    fn visit_expr(&mut self, expr: &Expr) -> fmt::Result {
        expr.accept(self)
    }

    fn visit_print(&mut self, expr: &Expr) -> fmt::Result {
        self.f.write_str("(print ")?;
        expr.accept(self)?;
        self.f.write_char(')')
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lisp<'a, 'b>(&'b Stmt<'a>);

impl<'a, 'b> Lisp<'a, 'b> {
    pub fn new(stmt: &'b Stmt<'a>) -> Self {
        Self(stmt)
    }
}

impl<'a, 'b> Display for Lisp<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.accept(&mut LispAstPrinter { f })
    }
}
