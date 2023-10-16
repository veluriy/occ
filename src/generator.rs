use crate::types::{Node, Token};

/// 構文木からアセンブリコードを再帰的に作る
pub fn print_assembly_by_node(node: &Node) {
    if let Token::Num(n) = node.kind {
        println!("  push {:?}", n);
        return;
    }
    if let Some(b) = &node.lhs {
        print_assembly_by_node(b);
    }
    if let Some(b) = &node.rhs {
        print_assembly_by_node(b);
    }
    println!("  pop rdi");
    println!("  pop rax");
    match node.kind {
        Token::Operand("+") => println!("  add rax, rdi"),
        Token::Operand("-") => println!("  sub rax, rdi"),
        Token::Operand("*") => println!("  imul rax, rdi"),
        Token::Operand("/") => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        _ => {}
    };
    println!("  push rax");
}
