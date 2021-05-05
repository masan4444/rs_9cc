use crate::{Token, TokenKind};
use std::{fmt, iter::Peekable};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Eof,
    UnexpectedToken(Token),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parser error")
    }
}
impl std::error::Error for Error {}

pub struct TokenIter<I: Iterator<Item = Token>>(Peekable<I>);
impl<I: Iterator<Item = Token>> TokenIter<I> {
    pub fn new(p: Peekable<I>) -> Self {
        Self(p)
    }
    pub fn expect_number(&mut self) -> std::result::Result<u64, Error> {
        let token = self.0.peek().ok_or(Error::Eof)?;
        match token.kind {
            TokenKind::Number(n) => {
                self.0.next();
                return Ok(n);
            }
            _ => return Err(Error::UnexpectedToken(*token)),
        }
    }
}
impl<I: Iterator<Item = Token>> Iterator for TokenIter<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
