use rs_9cc::strtol;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let mut chars = args
        .get(1)
        .ok_or("invalid number of arguments")?
        .chars()
        .peekable();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", strtol(&mut chars, 10).unwrap());

    while let Some(sign) = chars.next() {
        match sign {
            '+' => println!("  add rax, {}", strtol(&mut chars, 10).unwrap()),
            '-' => println!("  sub rax, {}", strtol(&mut chars, 10).unwrap()),
            c => return Err(format!("invalid char: {}", c))?,
        }
    }
    println!("  ret");
    Ok(())
}
