use std::{fmt::Display, io};

#[derive(Debug)]
pub enum InterpreterError {
    TooManyArgs,
    Io(io::Error),
    LexicalError(LexicalError),
    HadError,
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
            InterpreterError::HadError => f.write_str("Had and error"),
        }
    }
}

impl From<io::Error> for InterpreterError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, PartialEq)]
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
