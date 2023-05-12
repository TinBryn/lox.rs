pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
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
        if self.had_error {
            return Err(InterpreterError::HadError);
        }
        Ok(())
    }

    pub fn run(&mut self, script: &str) -> Result<(), InterpreterError> {
        let scanner = scanner::Scanner::new(script);
        for token in scanner {
            match token {
                Ok(token) => println!("{token:?}"),
                Err(err) => self.error(err.into()),
            }
        }
        Ok(())
    }

    pub fn error(&mut self, err: InterpreterError) {
        eprintln!("{err}");
        self.had_error = true;
    }
}

pub mod parser;
pub mod scanner;
pub mod syntax;
pub mod tokens;

use std::{io::stdin, path::Path};

use crate::error::InterpreterError;

#[cfg(test)]
mod test {
    use super::Lox;

    #[test]
    fn run_with_unexpected_char() {
        let bad_input = "{}#+-";
        let mut lox = Lox::new();
        assert!(!lox.had_error, "interpreter should not have an error yet");

        lox.run(bad_input).unwrap();

        assert!(lox.had_error, "interpreter should have had an error");
    }
}
