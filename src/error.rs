use std::{fmt::Display, io};

use crate::interpreter::tokens::{Operator, Structure};

#[derive(Debug)]
pub enum InterpreterError {
    TooManyArgs,
    Io(io::Error),
    LexicalError(LexicalError),
    ParserError(ParserError),
}

impl PartialEq for InterpreterError {
    fn eq(&self, other: &Self) -> bool {
        use InterpreterError::*;
        matches!((self, other), (TooManyArgs, TooManyArgs))
    }
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::TooManyArgs => f.write_str("Error: Too many arguments"),
            InterpreterError::Io(err) => f.write_fmt(format_args!("IoError: {err}")),
            InterpreterError::LexicalError(LexicalError::UnexpectedChar(char, row, col)) => f
                .write_fmt(format_args!(
                    "[{row}:{col}] LexicalError: Unexpected {char:?}"
                )),
            InterpreterError::LexicalError(LexicalError::UnterminatedString(row, col)) => f
                .write_fmt(format_args!(
                    "[{row}:{col}] starts a string that is not terminated"
                )),
            InterpreterError::LexicalError(LexicalError::ParseNumberError(row, col)) => {
                f.write_fmt(format_args!("[{row}:{col}] is an invalid number"))
            }
            InterpreterError::ParserError(err) => f.write_fmt(format_args!("{err:?}")),
        }
    }
}

impl From<io::Error> for InterpreterError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexicalError {
    UnexpectedChar(char, usize, usize),
    UnterminatedString(usize, usize),
    ParseNumberError(usize, usize),
}

impl From<LexicalError> for InterpreterError {
    fn from(value: LexicalError) -> Self {
        Self::LexicalError(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    LexicalError(LexicalError),
    Unsupported,
    BadOperator(Option<Operator>),
    BadStructure(Option<Structure>),
    EndOfFile,
}

impl From<LexicalError> for ParserError {
    fn from(value: LexicalError) -> Self {
        Self::LexicalError(value)
    }
}

impl From<ParserError> for InterpreterError {
    fn from(value: ParserError) -> Self {
        Self::ParserError(value)
    }
}
