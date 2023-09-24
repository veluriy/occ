use crate::types::{Node, Parser, Token};

/// パーサーを使ってアセンブリを作る
pub fn generate_assembly_by_parser(mut parser: Parser) {
    let node = parser.parse();
    if let Some(b) = node {
        print_assembly_by_node(&b);
    }
}

/// 構文木からアセンブリコードを再帰的に作る
fn print_assembly_by_node(node: &Node) {
    if let Token::Num(n) = node.kind {
        println!("  push {:?}", n);
        return;
    }
    if let Some(b) = &node.lhs {
        print_assembly_by_node(&b);
    }
    if let Some(b) = &node.rhs {
        print_assembly_by_node(&b);
    }
    println!("  pop rdi");
    println!("  pop rax");
    match node.kind {
        Token::Plus => println!("  add rax, rdi"),
        Token::Minus => println!("  sub rax, rdi"),
        Token::Mul => println!("  imul rax, rdi"),
        Token::Div => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        _ => {}
    };
    println!("  push rax");
}
