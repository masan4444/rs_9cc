use crate::{Token, TokenKind};
use std::{fmt, iter::Peekable};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Eof,
    UnexpectedToken(Token),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Eof => write!(f, "Eof"),
            Error::UnexpectedToken(token) => {
                let loc = token.loc();
                let padd = " ".repeat(loc.start);
                let allow = "^".repeat(loc.end - loc.start);
                write!(f, "{}{} unexpected token {:?}", padd, allow, token)
            }
        }
    }
}
impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;
pub struct TokenIter<I: Iterator<Item = Token>>(Peekable<I>);

pub trait TokenIterator: Iterator<Item = Token> {
    fn peek(&mut self) -> Result<Token>;
    fn expect_number(&mut self) -> Result<u64>;
    fn expect_byte(&mut self, b: u8) -> Result<()>;
}

impl<I: Iterator<Item = Token>> TokenIterator for TokenIter<I> {
    fn peek(&mut self) -> Result<Token> {
        Ok(*self.0.peek().ok_or(Error::Eof)?)
    }
    fn expect_number(&mut self) -> Result<u64> {
        let token = self.peek()?;
        match token.kind {
            TokenKind::Number(n) => {
                self.0.next();
                Ok(n)
            }
            _ => Err(Error::UnexpectedToken(token)),
        }
    }
    fn expect_byte(&mut self, b: u8) -> Result<()> {
        let token = self.peek()?;
        if token.kind == TokenKind::from(b) {
            self.0.next();
            Ok(())
        } else {
            Err(Error::UnexpectedToken(token))
        }
    }
}

impl<I: Iterator<Item = Token>> From<Peekable<I>> for TokenIter<I> {
    fn from(p: Peekable<I>) -> Self {
        Self(p)
    }
}

impl<I: Iterator<Item = Token>> Iterator for TokenIter<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Debug)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num(u64),
}
#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

impl Node {
    fn new(kind: NodeKind, lhs: Node, rhs: Node) -> Self {
        Self {
            kind,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
        }
    }
    fn new_num(n: u64) -> Self {
        Self {
            kind: NodeKind::Num(n),
            lhs: None,
            rhs: None,
        }
    }
    pub fn expr(tokens: &mut impl TokenIterator) -> Result<Self> {
        let mut node = Node::mul(tokens)?;
        loop {
            if let Ok(_) = tokens.expect_byte(b'+') {
                node = Node::new(NodeKind::Add, node, Node::mul(tokens)?);
            } else if let Ok(_) = tokens.expect_byte(b'-') {
                node = Node::new(NodeKind::Sub, node, Node::mul(tokens)?);
            } else {
                return Ok(node);
            }
        }
    }
    fn mul(tokens: &mut impl TokenIterator) -> Result<Self> {
        let mut node = Node::unary(tokens)?;
        loop {
            if let Ok(_) = tokens.expect_byte(b'*') {
                node = Node::new(NodeKind::Mul, node, Node::unary(tokens)?);
            } else if let Ok(_) = tokens.expect_byte(b'/') {
                node = Node::new(NodeKind::Div, node, Node::unary(tokens)?);
            } else {
                return Ok(node);
            }
        }
    }
    fn primary(tokens: &mut impl TokenIterator) -> Result<Self> {
        if let Ok(_) = tokens.expect_byte(b'(') {
            let node = Node::expr(tokens)?;
            tokens.expect_byte(b')')?;
            return Ok(node);
        } else {
            let node = Node::new_num(tokens.expect_number()?);
            return Ok(node);
        }
    }
    fn unary(tokens: &mut impl TokenIterator) -> Result<Self> {
        if let Ok(_) = tokens.expect_byte(b'+') {
            return Node::primary(tokens);
        } else if let Ok(_) = tokens.expect_byte(b'-') {
            return Ok(Node::new(
                NodeKind::Sub,
                Node::new_num(0),
                Node::primary(tokens)?,
            ));
        } else {
            return Node::primary(tokens);
        }
    }
}

pub fn parse<I: Iterator<Item = Token>>(tokens: Peekable<I>) -> Result<Node> {
    let mut token_iter: TokenIter<I> = tokens.into();
    Node::expr(&mut token_iter)
}
