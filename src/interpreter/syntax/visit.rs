use super::{Binary, Expr, Grouping, Literal, Unary};

pub trait Visitor<R> {
    fn visit_binary(&mut self, binary: &Binary) -> R;
    fn visit_group(&mut self, group: &Grouping) -> R;
    fn visit_literal(&mut self, lit: &Literal) -> R;
    fn visit_unary(&mut self, unary: &Unary) -> R;
}

impl<'a> Expr<'a> {
    /// The visitor pattern for this enum, implement the trait
    /// [`ExpressionVisitor<R>`] and pass it to this method.
    pub fn accept<R, V: Visitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(group) => visitor.visit_group(group),
            Expr::Literal(lit) => visitor.visit_literal(lit),
            Expr::Unary(unary) => visitor.visit_unary(unary),
        }
    }
}
