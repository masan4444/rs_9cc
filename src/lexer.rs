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
                b'0'..=b'9' => tokens.push_back(lex_number(&mut input)?),
                b'+' | b'-' | b'*' | b'/' | b'(' | b')' => {
                    tokens.push_back(lex_byte(&mut input, &[b])?)
                }
                b'=' => tokens.push_back(lex_byte(&mut input, &[b'=', b'='])?),
                b'!' => tokens.push_back(lex_byte(&mut input, &[b'!', b'='])?),
                b'<' => tokens.push_back(
                    lex_byte(&mut input, &[b'<', b'=']).or(lex_byte(&mut input, &[b'<']))?,
                ),
                b'>' => tokens.push_back(
                    lex_byte(&mut input, &[b'>', b'=']).or(lex_byte(&mut input, &[b'>']))?,
                ),
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
fn lex_byte(input: &mut Input, b: &[u8]) -> Result<Token> {
    input
        .consume_bytes(&b)
        .map(|pos| Token::new(b.into(), pos..input.pos()))
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

#[derive(Clone, Copy, PartialEq)]
pub enum TokenKind {
    Number(u64),
    Plus,     // '+'
    Minus,    // '-'
    Asterisk, // '*'
    Slash,    // '/'
    LParen,   // '('
    RParen,   // ')'
    Lt,       // '<'
    Leq,      // '<='
    Gt,       // '>'
    Geq,      // '>='
    Eq,       // '=='
    Neq,      // '!='
    Eof,
}

impl From<&[u8]> for TokenKind {
    fn from(value: &[u8]) -> Self {
        use TokenKind::*;
        match value {
            [b'<', b'='] => Leq,
            [b'>', b'='] => Geq,
            [b'=', b'='] => Eq,
            [b'!', b'='] => Neq,
            [b'<'] => Lt,
            [b'>'] => Gt,

            [b'+'] => Plus,
            [b'-'] => Minus,
            [b'*'] => Asterisk,
            [b'/'] => Slash,
            [b'('] => LParen,
            [b')'] => RParen,
            _ => panic!(),
        }
    }
}

impl Into<String> for TokenKind {
    fn into(self) -> String {
        use TokenKind::*;
        match self {
            Number(n) => format!("Number({})", n),
            Plus => "Plus('+')".to_owned(),
            Minus => "Minus('-')".to_owned(),
            Asterisk => "Asterisk('*')".to_owned(),
            Slash => "Slash('/')".to_owned(),
            LParen => "Lparen('(')".to_owned(),
            RParen => "Rparen(')')".to_owned(),
            Lt => "Lt('<')".to_owned(),
            Leq => "Leq('<=')".to_owned(),
            Gt => "Gt('>')".to_owned(),
            Geq => "Geq('>=')".to_owned(),
            Eq => "Eq('==')".to_owned(),
            Neq => "Neq('!=')".to_owned(),
            Eof => "Eof".to_owned(),
        }
    }
}

impl fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}

pub type Token = Annot<TokenKind>;

impl Token {
    fn new(token_kind: TokenKind, loc: impl Into<Loc>) -> Self {
        Token {
            kind: token_kind,
            loc: loc.into(),
        }
    }
    pub fn loc(&self) -> Range<usize> {
        self.loc.0..self.loc.1
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
    fn peek_nth(&self, n: usize) -> Result<u8> {
        self.s.get(self.pos() + n).map(|&b| b).ok_or(self.eof())
    }
    fn inc(&mut self) {
        self.pos += 1
    }
    fn inc_nth(&mut self, n: usize) {
        self.pos += n
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
    fn consume_bytes(&mut self, wants: &[u8]) -> Result<usize> {
        let pos = self.pos();
        wants
            .iter()
            .enumerate()
            .try_for_each(|(idx, &want)| {
                self.peek_nth(idx).and_then(|got| {
                    (got == want).then(|| ()).ok_or(Error::new(
                        ErrorKind::InvalidChar(got as char),
                        (pos + idx)..(pos + idx + 1),
                    ))
                })
            })
            .map(|_| {
                self.inc_nth(wants.len());
                pos
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
