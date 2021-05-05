use std::{collections::LinkedList, fmt, ops::Range};

pub fn tokenize(s: &str) -> std::result::Result<LinkedList<Token>, Error> {
    let mut input = Input::new(s);
    let mut tokens = LinkedList::new();

    loop {
        match input.peek() {
            Err(e) => match e.kind {
                ErrorKind::Eof => {
                    tokens.push_back(Token::eof(input.pos()..input.pos()));
                    return Ok(tokens);
                }
                _ => return Err(e),
            },
            Ok(b) => match b {
                b'0'..=b'9' => {
                    let token = lex_number(&mut input)?;
                    tokens.push_back(token);
                }
                b'+' => {
                    let token = lex_plus(&mut input)?;
                    tokens.push_back(token);
                }
                b'-' => {
                    let token = lex_minus(&mut input)?;
                    tokens.push_back(token);
                }
                b' ' => input.consume_spaces(),
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidChar(b as char),
                        input.pos()..input.pos() + 1,
                    ))
                }
            },
        }
    }
}

fn lex_number(input: &mut Input) -> Result<Token> {
    input
        .consume_numbers()
        .map(|(pos, n)| Token::new(TokenKind::Number(n), pos..input.pos()))
}
fn lex_plus(input: &mut Input) -> Result<Token> {
    input
        .consume_byte(b'+')
        .map(|pos| Token::new(TokenKind::Plus, pos..input.pos()))
}
fn lex_minus(input: &mut Input) -> Result<Token> {
    input
        .consume_byte(b'-')
        .map(|pos| Token::new(TokenKind::Minus, pos..input.pos()))
}

#[derive(Debug, Clone, Copy)]
struct Loc(usize, usize);
impl From<Range<usize>> for Loc {
    fn from(range: Range<usize>) -> Self {
        Self(range.start, range.end)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Annot<T> {
    pub kind: T,
    loc: Loc,
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidChar(char),
    Eof,
}

pub type Error = Annot<ErrorKind>;
type Result<T> = std::result::Result<T, Error>;

impl Error {
    fn new(error_kind: ErrorKind, loc: impl Into<Loc>) -> Self {
        Error {
            kind: error_kind,
            loc: loc.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;
        match self.kind {
            InvalidChar(c) => {
                let padd = " ".repeat(self.loc.0);
                let allow = "^".repeat(self.loc.1 - self.loc.0);
                write!(f, "{}{} invalid char '{}'", padd, allow, c)
            }
            _ => write!(f, "lex error"),
        }
    }
}

impl std::error::Error for Error {}

struct Input<'a> {
    s: &'a [u8],
    pos: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Number(u64),
    Plus,
    Minus,
    Eof,
}

pub type Token = Annot<TokenKind>;

impl Token {
    fn new(token_kind: TokenKind, loc: impl Into<Loc>) -> Self {
        Token {
            kind: token_kind,
            loc: loc.into(),
        }
    }
    fn eof(loc: impl Into<Loc>) -> Self {
        Token {
            kind: TokenKind::Eof,
            loc: loc.into(),
        }
    }
}

impl<'a> Input<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            s: s.as_bytes(),
            pos: 0,
        }
    }
    fn eof(&self) -> Error {
        let pos = self.pos();
        Error::new(ErrorKind::Eof, pos..pos)
    }
    fn pos(&self) -> usize {
        self.pos
    }
    fn peek(&self) -> Result<u8> {
        self.s.get(self.pos()).map(|&b| b).ok_or(self.eof())
    }
    fn inc(&mut self) {
        self.pos += 1
    }
    fn pos_then_inc(&mut self) -> usize {
        let pos = self.pos();
        self.inc();
        pos
    }
    fn consume(&mut self, f: fn(u8) -> bool) {
        while let Ok(b) = self.peek() {
            if f(b) {
                self.inc();
                continue;
            }
            break;
        }
    }
    fn consume_byte(&mut self, want: u8) -> Result<usize> {
        self.peek().and_then(|got| {
            if got != want {
                let pos = self.pos();
                Err(Error::new(
                    ErrorKind::InvalidChar(got as char),
                    pos..pos + 1,
                ))
            } else {
                Ok(self.pos_then_inc())
            }
        })
    }
    fn consume_numbers(&mut self) -> Result<(usize, u64)> {
        let start = self.pos();
        self.consume(|c| (c as char).is_ascii_digit());
        let n = std::str::from_utf8(&self.s[start..self.pos()])
            .unwrap()
            .parse()
            .unwrap();
        Ok((start, n))
    }
    fn consume_spaces(&mut self) {
        self.consume(|c| (c as char).is_ascii_whitespace())
    }
}
