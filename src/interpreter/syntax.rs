use super::tokens::Operator;

#[derive(Debug, Clone)]
pub struct Binary<'a> {
    pub left: Expr<'a>,
    pub operator: Operator,
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
    operator: Operator,
    expression: Expr<'a>,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary(Box<Binary<'a>>),
    Grouping(Box<Grouping<'a>>),
    Literal(Literal<'a>),
    Unary(Box<Unary<'a>>),
}

impl<'a> Expr<'a> {
    pub fn from_binary(left: Self, operator: Operator, right: Self) -> Self {
        Expr::Binary(Box::new(Binary {
            left,
            operator,
            right,
        }))
    }
    pub fn from_grouping(expression: Self) -> Self {
        Self::Grouping(Box::new(Grouping { expression }))
    }
    pub fn from_unary(operator: Operator, expression: Self) -> Self {
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

mod visit;

mod printer;

#[cfg(test)]
mod test {
    use crate::interpreter::tokens::Operator;

    use super::{printer::SExpr, Expr};

    #[test]
    fn debug_expression_tree() {
        let e1 = Expr::from_unary(Operator::Minus, Expr::from_number(123.));
        let e2 = Expr::from_grouping(Expr::from_number(45.67));
        let expr = Expr::from_binary(e1, Operator::Star, e2);
        let s_expr = SExpr::new(&expr);

        let expected = "(* (- 123) (group 45.67))";

        assert_eq!(format!("{s_expr}"), expected);
    }
}
