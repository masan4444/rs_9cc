use rs_9cc::{tokenize, AstError, LexError, TokenIter, TokenKind};
use std::{
    env,
    fmt::{self, Display},
    process,
};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let s = args.get(1).ok_or("invalid number of arguments")?;

    match run(s) {
        Ok(asm) => println!("{}", asm),
        Err(e) => {
            eprintln!("{}", s);
            eprintln!("{}", e);
            process::exit(1)
        }
    }
    Ok(())
}

#[derive(Debug)]
enum Error {
    Lexer(LexError),
    Parser(AstError),
}

impl From<AstError> for Error {
    fn from(e: AstError) -> Self {
        Error::Parser(e)
    }
}

impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lexer(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lexer(e) => e.fmt(f),
            Error::Parser(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

fn run(s: &str) -> std::result::Result<String, Error> {
    let tokens = tokenize(s)?;
    let mut token_iter = TokenIter::new(tokens.into_iter().peekable());

    let mut asm = String::new();
    asm.push_str(".intel_syntax noprefix");
    asm.push_str(".globl main");
    asm.push_str("main:");
    asm.push_str(&format!("  mov rax, {}", token_iter.expect_number()?));

    while let Some(token) = token_iter.next() {
        match token.kind {
            TokenKind::Plus => asm.push_str(&format!("  add rax, {}", token_iter.expect_number()?)),
            TokenKind::Minus => {
                asm.push_str(&format!("  sub rax, {}", token_iter.expect_number()?))
            }
            TokenKind::Eof => break,
            _ => return Err(AstError::Eof)?,
        }
    }
    println!("  ret");
    Ok(asm)
}
