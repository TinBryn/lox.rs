use super::{Binary, Expr, Grouping, Literal, Unary, Stmt};

pub trait ExprVisitor<R> {
    fn visit_binary(&mut self, binary: &Binary) -> R;
    fn visit_group(&mut self, group: &Grouping) -> R;
    fn visit_literal(&mut self, lit: &Literal) -> R;
    fn visit_unary(&mut self, unary: &Unary) -> R;
}

pub trait StmtVisitor<R> {
    fn visit_expr(&mut self, expr: &Expr) -> R;
    fn visit_print(&mut self, expr: &Expr) -> R;
}

impl<'a> Stmt<'a> {
    pub fn accept<R, V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr(expr),
            Stmt::Print(expr) => visitor.visit_print(expr),
        }
    }
}

impl<'a> Expr<'a> {
    /// The visitor pattern for this enum, implement the trait
    /// [`Visitor<R>`] and pass it to this method.
    pub fn accept<R, V: ExprVisitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(group) => visitor.visit_group(group),
            Expr::Literal(lit) => visitor.visit_literal(lit),
            Expr::Unary(unary) => visitor.visit_unary(unary),
        }
    }
}
