use rs_9cc::{generate, parse, tokenize, Error};
use std::{env, process};

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

fn run(s: &str) -> std::result::Result<String, Error> {
    let tokens = tokenize(s)?;
    // println!("{:?}", tokens);
    let ast = parse(tokens.into_iter().peekable())?;
    // println!("{:?}", ast);
    let asm = generate(&ast);

    Ok(asm)
}
