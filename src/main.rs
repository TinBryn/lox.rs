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

use interpreter::Lox;

fn main() -> Result<(), error::InterpreterError> {
    let args: Vec<_> = std::env::args().collect();
    match &args[..] {
        [] => Lox::run_prompt().map_err(Into::into),
        [script] => Lox::run_file(script).map_err(Into::into),
        _ => {
            eprintln!("Usage: lox [script]");
            Err(error::InterpreterError::TooManyArgs)
        }
    }
}

// Basic data types
pub enum BasicData {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

mod error;
mod interpreter;
