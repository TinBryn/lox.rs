use super::tokens::Keyword;

#[derive(Debug, Clone)]
pub struct Binary<'a> {
    pub left: Expr<'a>,
    pub operator: Keyword,
    pub right: Expr<'a>,
}

#[derive(Debug, Clone)]
pub struct Grouping<'a> {
    pub expression: Expr<'a>,
}

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    String(&'a str),
    Number(f64),
    True,
    False,
    Nil,
}

#[derive(Debug, Clone)]
pub struct Unary<'a> {
    operator: Keyword,
    expresion: Expr<'a>,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary(Box<Binary<'a>>),
    Grouping(Box<Grouping<'a>>),
    Literal(Literal<'a>),
    Unary(Box<Unary<'a>>),
}
