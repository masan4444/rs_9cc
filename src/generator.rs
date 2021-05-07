use crate::{Node, NodeKind};

fn gen(node: &Node, asm: &mut String) {
    if let NodeKind::Num(n) = node.kind {
        asm.push_str(&format!("  push {}\n", n));
        return;
    }

    gen(node.lhs.as_ref().unwrap(), asm);
    gen(node.rhs.as_ref().unwrap(), asm);

    asm.push_str("  pop rdi\n");
    asm.push_str("  pop rax\n");

    use NodeKind::*;
    match node.kind {
        Add => asm.push_str("  add rax, rdi\n"),
        Sub => asm.push_str("  sub rax, rdi\n"),
        Mul => asm.push_str("  imul rax, rdi\n"),
        Div => asm.push_str("  cqo\n  idiv rdi\n"),
        Num(_) => {}
    }

    asm.push_str("  push rax\n")
}

pub fn generate(ast: &Node) -> String {
    let mut asm = String::new();

    asm.push_str(".intel_syntax noprefix\n");
    asm.push_str(".globl main\n");
    asm.push_str("main:\n");

    gen(ast, &mut asm);

    asm.push_str("  pop rax\n");
    asm.push_str("  ret\n");
    asm
}
