//! # Lox
//!
//! arithmatic operators work on numbers
//!
//! comparison operators always return a boolean
//!
//! equality of different types is always false
//!
//! against implicit conversions
//!
//! logical operators work on booleans, (`!`, `and`, `or`)
//!
//! boolean operators short-circuit
//!
//! `var` to declare variables
//!
//! `fun` declares functions

use std::{io::stdin, path::Path};

use interpreter::Interpreter;

use crate::{error::InterpreterError, parser::Parser};

fn main() -> Result<(), error::InterpreterError> {
    let mut lox = Lox::new();
    let args: Vec<_> = std::env::args().collect();
    match &args[..] {
        [] => lox.run_prompt().map_err(Into::into),
        [script] => lox.run_file(script).map_err(Into::into),
        _ => {
            eprintln!("Usage: lox [script]");
            Err(error::InterpreterError::TooManyArgs)
        }
    }
}

pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }
    pub fn run_prompt(&mut self) -> Result<(), InterpreterError> {
        loop {
            print!("> ");
            let mut line = Default::default();
            stdin().read_line(&mut line)?;
            if line.is_empty() {
                return Ok(());
            }
            self.run(&line)?;
        }
    }

    pub fn run_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), InterpreterError> {
        let data = std::fs::read_to_string(path)?;
        self.run(&data)?;

        Ok(())
    }

    pub fn run(&mut self, script: &str) -> Result<(), InterpreterError> {
        let mut parser = Parser::new(script);
        let expr = parser.parse()?;
        println!("{}", expr.display_lisp());
        let value = self.interpreter.evaluate(&expr)?;
        println!("{:?}", value);

        Ok(())
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {

    use super::Lox;

    #[test]
    fn run_with_unexpected_char() {
        let mut lox = Lox::new();
        let bad_input = "{}#+-";
        let result = lox.run(bad_input);
        println!("{:?}", result);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn example_expression() {
        let mut lox = Lox::new();
        let input = "1 + 2 * 3 == 7";
        lox.run(input).unwrap();
    }
}

mod error;
mod interpreter;
mod parser;
mod scanner;
mod syntax;
mod token;
mod value;
