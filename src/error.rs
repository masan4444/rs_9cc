use crate::{LexError, ParseError};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Lexer(LexError),
    Parser(ParseError),
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parser(e)
    }
}

impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lexer(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lexer(e) => e.fmt(f),
            Error::Parser(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
