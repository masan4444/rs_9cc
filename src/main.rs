use std::{env, error::Error, iter::Peekable};

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

fn strtol<I: Iterator<Item = char>>(iter: &mut Peekable<I>, radix: u32) -> Option<u32> {
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
