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
        Eq | Neq | Lt | Leq => asm.push_str(&cmp(&node.kind)),
        Num(_) => panic!(),
    }

    asm.push_str("  push rax\n")
}

fn cmp(node_kind: &NodeKind) -> String {
    use NodeKind::*;
    let mut asm = String::from("  cmp rax, rdi\n");
    asm.push_str("  ");
    asm.push_str(match node_kind {
        Eq => "sete",
        Neq => "setne",
        Lt => "setl",
        Leq => "setle",
        _ => panic!(),
    });
    asm.push_str(" al\n");
    asm.push_str("  movzb rax, al\n");
    asm
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
