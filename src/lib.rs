mod ast;
mod error;
mod lex;

pub use ast::{Error as AstError, TokenIter};
pub use error::Error;
pub use lex::{tokenize, Error as LexError, Token, TokenKind};

use std::iter::Peekable;

pub fn strtol<I: Iterator<Item = char>>(iter: &mut Peekable<I>, radix: u32) -> Option<u32> {
    let mut init = iter.peek()?.to_digit(radix)?;
    iter.next();
    while let Some(c) = iter.peek() {
        if let Some(n) = c.to_digit(radix) {
            init = init * radix + n;
            iter.next();
        } else {
            break;
        }
    }
    Some(init)
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn it_works() {
        Command::new("./test.sh").assert().success();
    }
}
