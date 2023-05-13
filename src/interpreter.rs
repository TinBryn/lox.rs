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
        println!("{}", expr.as_ast());

        Ok(())
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}

mod parser;
mod scanner;
mod syntax;
pub mod tokens;

use std::{io::stdin, path::Path};

use crate::error::InterpreterError;

use self::parser::Parser;

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
