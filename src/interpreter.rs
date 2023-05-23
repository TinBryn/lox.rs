use crate::{
    error::InterpreterError,
    syntax::{
        self,
        visit::{ExprVisitor, StmtVisitor},
        BinOp, Expr, Literal, Stmt, UnOp,
    },
    value::Value,
};

#[derive(Debug, Default, Clone)]
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), InterpreterError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
        stmt.accept(self)
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        expr.accept(self)
    }

    fn numeric(value: Value) -> Result<f64, InterpreterError> {
        match value {
            Value::Number(n) => Ok(n),
            value => Err(InterpreterError::TypeError(value)),
        }
    }

    fn numeric_op<F: FnOnce(f64, f64) -> f64>(
        left: Value,
        right: Value,
        f: F,
    ) -> Result<Value, InterpreterError> {
        Ok(f(Self::numeric(left)?, Self::numeric(right)?).into())
    }

    fn cmp_op<F: FnOnce(f64, f64) -> bool>(
        left: Value,
        right: Value,
        f: F,
    ) -> Result<Value, InterpreterError> {
        Ok(f(Self::numeric(left)?, Self::numeric(right)?).into())
    }

    fn eq(left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::String(left), Value::String(right)) => left == right,
            (Value::Number(left), Value::Number(right)) => left == right,
            (Value::Bool(left), Value::Bool(right)) => left == right,
            (Value::Nil, Value::Nil) => true,

            _ => false,
        }
    }

    fn truthy(value: &Value) -> bool {
        !matches!(*value, Value::Nil | Value::Bool(false))
    }
}

impl StmtVisitor<Result<(), InterpreterError>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<(), InterpreterError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<(), InterpreterError> {
        let value = self.evaluate(expr)?;
        println!("{value}");
        Ok(())
    }

    #[allow(unused_variables)]
    fn visit_var(&mut self, name: &str, expr: &Expr) -> Result<(), InterpreterError> {
        todo!()
    }
}

impl ExprVisitor<Result<Value, InterpreterError>> for Interpreter {
    fn visit_binary(&mut self, binary: &syntax::Binary) -> Result<Value, InterpreterError> {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;
        match binary.operator {
            BinOp::Add => match (left, right) {
                (Value::Number(left), Value::Number(right)) => Ok((left + right).into()),
                (Value::String(left), Value::String(right)) => Ok((left + &right).into()),
                (left, Value::Number(_) | Value::String(_)) => {
                    Err(InterpreterError::TypeError(left))
                }
                (_, right) => Err(InterpreterError::TypeError(right)),
            },
            BinOp::Sub => Self::numeric_op(left, right, |l, r| l - r),
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

    fn visit_group(&mut self, group: &syntax::Grouping) -> Result<Value, InterpreterError> {
        self.evaluate(&group.expression)
    }

    fn visit_literal(&mut self, lit: &Literal) -> Result<Value, InterpreterError> {
        match *lit {
            Literal::String(ref s) => Ok(Value::String(s.clone())),
            Literal::Number(n) => Ok(Value::Number(n)),
            Literal::True => Ok(Value::Bool(true)),
            Literal::False => Ok(Value::Bool(false)),
            Literal::Nil => Ok(Value::Nil),
            Literal::Identifier(_) => todo!(),
        }
    }

    fn visit_unary(&mut self, unary: &syntax::Unary) -> Result<Value, InterpreterError> {
        let value = self.evaluate(&unary.expression)?;

        match unary.operator {
            UnOp::Neg => {
                let n = Self::numeric(value)?;
                Ok(Value::Number(-n))
            }
            UnOp::Not => {
                let b = Self::truthy(&value);
                Ok(Value::Bool(!b))
            }
        }
    }
}
