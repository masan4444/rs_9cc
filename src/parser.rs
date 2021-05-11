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
            Error::Eof => write!(f, "Parser Error: end of file reached"),
            Error::UnexpectedToken(token) => {
                let loc = token.loc();
                let padd = " ".repeat(loc.start);
                let allow = "^".repeat(loc.end - loc.start);
                write!(
                    f,
                    "{}{} Parser Error: unexpected token {:?}",
                    padd, allow, token
                )
            }
        }
    }
}
impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;
struct TokenIter<I: Iterator<Item = Token>>(Peekable<I>);

trait TokenIterator: Iterator<Item = Token> {
    fn peek(&mut self) -> Result<Token>;
    fn expect_number(&mut self) -> Result<u64>;
    fn expect_token(&mut self, token_kind: TokenKind) -> Result<()>;
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
    fn expect_token(&mut self, token_kind: TokenKind) -> Result<()> {
        let token = self.peek()?;
        if token.kind == token_kind {
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
    Eq,
    Neq,
    Lt,
    Leq,
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
    fn expr(tokens: &mut impl TokenIterator) -> Result<Self> {
        Node::equality(tokens)
    }
    fn equality(tokens: &mut impl TokenIterator) -> Result<Self> {
        let mut node = Node::relational(tokens)?;
        loop {
            if let Ok(_) = tokens.expect_token(TokenKind::Eq) {
                node = Node::new(NodeKind::Eq, node, Node::relational(tokens)?)
            } else if let Ok(_) = tokens.expect_token(TokenKind::Neq) {
                node = Node::new(NodeKind::Neq, node, Node::relational(tokens)?);
            } else {
                return Ok(node);
            }
        }
    }
    fn relational(tokens: &mut impl TokenIterator) -> Result<Self> {
        let mut node = Node::add(tokens)?;
        loop {
            if let Ok(_) = tokens.expect_token(TokenKind::Lt) {
                node = Node::new(NodeKind::Lt, node, Node::add(tokens)?)
            } else if let Ok(_) = tokens.expect_token(TokenKind::Leq) {
                node = Node::new(NodeKind::Leq, node, Node::add(tokens)?);
            } else if let Ok(_) = tokens.expect_token(TokenKind::Gt) {
                node = Node::new(NodeKind::Lt, Node::add(tokens)?, node);
            } else if let Ok(_) = tokens.expect_token(TokenKind::Geq) {
                node = Node::new(NodeKind::Leq, Node::add(tokens)?, node);
            } else {
                return Ok(node);
            }
        }
    }
    fn add(tokens: &mut impl TokenIterator) -> Result<Self> {
        let mut node = Node::mul(tokens)?;
        loop {
            if let Ok(_) = tokens.expect_token(TokenKind::Plus) {
                node = Node::new(NodeKind::Add, node, Node::mul(tokens)?);
            } else if let Ok(_) = tokens.expect_token(TokenKind::Minus) {
                node = Node::new(NodeKind::Sub, node, Node::mul(tokens)?);
            } else {
                return Ok(node);
            }
        }
    }
    fn mul(tokens: &mut impl TokenIterator) -> Result<Self> {
        let mut node = Node::unary(tokens)?;
        loop {
            if let Ok(_) = tokens.expect_token(TokenKind::Asterisk) {
                node = Node::new(NodeKind::Mul, node, Node::unary(tokens)?);
            } else if let Ok(_) = tokens.expect_token(TokenKind::Slash) {
                node = Node::new(NodeKind::Div, node, Node::unary(tokens)?);
            } else {
                return Ok(node);
            }
        }
    }
    fn unary(tokens: &mut impl TokenIterator) -> Result<Self> {
        if let Ok(_) = tokens.expect_token(TokenKind::Plus) {
            return Node::primary(tokens);
        } else if let Ok(_) = tokens.expect_token(TokenKind::Minus) {
            return Ok(Node::new(
                NodeKind::Sub,
                Node::new_num(0),
                Node::primary(tokens)?,
            ));
        } else {
            return Node::primary(tokens);
        }
    }
    fn primary(tokens: &mut impl TokenIterator) -> Result<Self> {
        if let Ok(_) = tokens.expect_token(TokenKind::LParen) {
            let node = Node::expr(tokens)?;
            tokens.expect_token(TokenKind::RParen)?;
            return Ok(node);
        } else {
            let node = Node::new_num(tokens.expect_number()?);
            return Ok(node);
        }
    }
}

pub fn parse<I: Iterator<Item = Token>>(tokens: Peekable<I>) -> Result<Node> {
    let mut token_iter: TokenIter<I> = tokens.into();
    Node::expr(&mut token_iter)
}
