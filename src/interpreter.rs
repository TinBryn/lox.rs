pub struct Lox;

impl Lox {
    pub fn new() -> Self {
        Self
    }
    pub fn run_prompt() -> Result<(), InterpreterError> {
        loop {
            print!("> ");
            let mut line = Default::default();
            stdin().read_line(&mut line)?;
            if line.is_empty() {
                return Ok(());
            }
            Self::run(&line)?;
        }
    }

    pub fn run_file<P: AsRef<Path>>(path: P) -> Result<(), InterpreterError> {
        let data = std::fs::read_to_string(path)?;
        Self::run(&data)?;

        Ok(())
    }

    pub fn run(script: &str) -> Result<(), InterpreterError> {
        let mut parser = Parser::new(script);
        let expr = parser.parse()?;
        println!("{}", expr.display_lisp());

        Ok(())
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum LoxValue {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl From<&str> for LoxValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for LoxValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<f64> for LoxValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for LoxValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&mut self, expr: &Expr<'_>) -> Result<LoxValue, InterpreterError> {
        expr.accept(self)
    }

    fn numeric(value: LoxValue) -> Result<f64, InterpreterError> {
        match value {
            LoxValue::Number(n) => Ok(n),
            value => Err(InterpreterError::TypeError(value)),
        }
    }

    fn numeric_op<F: FnOnce(f64, f64) -> f64>(
        left: LoxValue,
        right: LoxValue,
        f: F,
    ) -> Result<LoxValue, InterpreterError> {
        Ok(f(Self::numeric(left)?, Self::numeric(right)?).into())
    }

    fn cmp_op<F: FnOnce(f64, f64) -> bool>(
        left: LoxValue,
        right: LoxValue,
        f: F,
    ) -> Result<LoxValue, InterpreterError> {
        Ok(f(Self::numeric(left)?, Self::numeric(right)?).into())
    }

    fn eq(left: &LoxValue, right: &LoxValue) -> bool {
        match (left, right) {
            (LoxValue::String(left), LoxValue::String(right)) => left == right,
            (LoxValue::Number(left), LoxValue::Number(right)) => left == right,
            (LoxValue::Bool(left), LoxValue::Bool(right)) => left == right,
            (LoxValue::Nil, LoxValue::Nil) => true,

            _ => false,
        }
    }

    fn truthy(value: &LoxValue) -> bool {
        !matches!(*value, LoxValue::Nil | LoxValue::Bool(false))
    }
}

impl Visitor<Result<LoxValue, InterpreterError>> for Interpreter {
    fn visit_binary(&mut self, binary: &syntax::Binary) -> Result<LoxValue, InterpreterError> {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;
        match binary.operator {
            BinOp::Sub => Self::numeric_op(left, right, |l, r| l - r),
            BinOp::Add => match (left, right) {
                (LoxValue::Number(left), LoxValue::Number(right)) => Ok((left + right).into()),
                (LoxValue::String(left), LoxValue::String(right)) => Ok((left + &right).into()),
                (left, _) => Err(InterpreterError::TypeError(left)),
            },
            BinOp::Div => Self::numeric_op(left, right, |l, r| l / r),
            BinOp::Mul => Self::numeric_op(left, right, |l, r| l * r),

            BinOp::Ne => Ok((!Self::eq(&left, &right)).into()),
            BinOp::Eq => Ok((Self::eq(&left, &right)).into()),

            BinOp::Gt => Self::cmp_op(left, right, |l, r| l > r),
            BinOp::Ge => Self::cmp_op(left, right, |l, r| l >= r),
            BinOp::Lt => Self::cmp_op(left, right, |l, r| l < r),
            BinOp::Le => Self::cmp_op(left, right, |l, r| l <= r),

            BinOp::And => {
                let b = Self::truthy(&left) && Self::truthy(&right);
                Ok(b.into())
            }
            BinOp::Or => {
                let b = Self::truthy(&left) || Self::truthy(&right);
                Ok(b.into())
            }
        }
    }

    fn visit_group(&mut self, group: &syntax::Grouping) -> Result<LoxValue, InterpreterError> {
        self.evaluate(&group.expression)
    }

    fn visit_literal(&mut self, lit: &syntax::Literal) -> Result<LoxValue, InterpreterError> {
        match *lit {
            syntax::Literal::String(s) => Ok(LoxValue::String(s.into())),
            syntax::Literal::Number(n) => Ok(LoxValue::Number(n)),
            syntax::Literal::True => Ok(LoxValue::Bool(true)),
            syntax::Literal::False => Ok(LoxValue::Bool(false)),
            syntax::Literal::Nil => Ok(LoxValue::Nil),
            syntax::Literal::Identifier(_) => todo!(),
        }
    }

    fn visit_unary(&mut self, unary: &syntax::Unary) -> Result<LoxValue, InterpreterError> {
        let value = self.evaluate(&unary.expression)?;

        match unary.operator {
            syntax::UnOp::Neg => {
                let n = Self::numeric(value)?;
                Ok(LoxValue::Number(-n))
            }
            syntax::UnOp::Not => {
                let b = Self::truthy(&value);
                Ok(LoxValue::Bool(!b))
            }
        }
    }
}

// mod parser;
// mod scanner;
// mod syntax;
// pub mod tokens;

use std::{io::stdin, path::Path};

use crate::{error::InterpreterError, syntax};

use crate::{
    syntax::{visit::Visitor, BinOp, Expr},
};
use crate::parser::Parser;

#[cfg(test)]
mod test {

    use super::Lox;

    #[test]
    fn run_with_unexpected_char() {
        let bad_input = "{}#+-";
        let result = Lox::run(bad_input);
        println!("{:?}", result);
        assert!(matches!(result, Err(_)));
    }
}
