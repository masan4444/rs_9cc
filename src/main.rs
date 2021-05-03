use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("invalid argument count");
        std::process::exit(1);
    }
    let arg: u8 = args[1].parse()?;
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", arg);
    println!("  ret");
    Ok(())
}
