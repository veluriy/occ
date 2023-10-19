use crate::types::{Node, Token, Variables};

fn generate_lvar(node: &Node, vars: &Variables) {
    // 変数であるかを見る
    if let Token::LVar(name) = node.kind {
        // オフセットを取得
        if let Some(offset) = vars.offsets.get(name) {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", offset);
            println!("  push rax");
        }
        else {
            panic!("the offset cannot be found.")
        }
    } else {
        panic!("")
    }
}

pub fn print_assembly_by_node(node: &Node, vars: &Variables) {
    match node.kind {
        Token::Num(n) => {
            println!("  push {}", n);
            return;
        },
        Token::LVar(name) => {
            generate_lvar(node, vars);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
        },
        Token::Operand(op) => {
            // '='の時は特別に、左辺値の扱いが他の二項演算と異なる。
            if op == "=" {
                if let Some(b) = &node.lhs {
                    generate_lvar(b, vars);
                }
                if let Some(b) = &node.rhs {
                    print_assembly_by_node(b, vars);
                }   
                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi"); 
                return;         
            }
            // operand ... 二項
            if let Some(b) = &node.lhs {
                print_assembly_by_node(b, vars);
            }
            if let Some(b) = &node.rhs {
                print_assembly_by_node(b, vars);
            }
            println!("  pop rdi");
            println!("  pop rax");
            match op {
                "+" => {
                    println!("  add rax, rdi");
                    println!("  push rax");
                },
                "-" => {
                    println!("  sub rax, rdi");
                    println!("  push rax");
                },
                "*" => {
                    println!("  imul rax, rdi");
                    println!("  push rax");
                },
                "/" => {
                    println!("  cqo");
                    println!("  idiv rdi");
                    println!("  push rax");
                },
                "<" => {
                    println!("  cmp rax, rdi");
                    println!("  setl al");
                    println!("  movzb rax, al"); 
                    println!("  push rax");    
                }
                "<=" => {
                    println!("  cmp rax, rdi");
                    println!("  setle al");
                    println!("  movzb rax, al");
                    println!("  push rax");
                },
                "!=" => {
                    println!("  cmp rax, rdi");
                    println!("  setne al");
                    println!("  movzb rax, al");
                    println!("  push rax");
                },
                "==" => {
                    println!("  cmp rax, rdi");
                    println!("  sete al");
                    println!("  movzb rax, al");
                    println!("  push rax");
                }
                _ => {
                    //
                }
            }
        },
        Token::Reserved(name) => {
            // 未実装
        },
    }
}