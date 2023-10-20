use crate::types::{Node, Token, Variables};

fn generate_lvar(node: &Node, vars: &Variables, assembly: &mut String) {
    // 変数であるかを見る
    if let Token::LVar(name) = node.kind {
        // オフセットを取得
        if let Some(offset) = vars.offsets.get(name) {
            assembly.push_str("\tmov rax, rbp\n");
            assembly.push_str(&format!("\tsub rax, {}\n", offset));
            assembly.push_str("\tpush rax\n");
        } else {
            panic!("the offset cannot be found.")
        }
    } else {
        panic!("")
    }
}

pub fn print_assembly_by_node(node: &Node, vars: &Variables, assembly: &mut String) {
    match node.kind {
        Token::Num(n) => {
            assembly.push_str(&format!("\tpush {}\n", n));
        }
        Token::LVar(_name) => {
            generate_lvar(node, vars, assembly);
            assembly.push_str("\tpop rax\n");
            assembly.push_str("\tmov rax, [rax]\n");
            assembly.push_str("\tpush rax\n");
        }
        Token::Reserved("return") => {
            if let Some(b) = &node.lhs {
                match b.kind {
                    Token::LVar(_) => {
                        generate_lvar(b, vars, assembly);
                    }
                    _ => {
                        print_assembly_by_node(b, vars, assembly);
                    }
                }
            }
            assembly.push_str("\tpop rax\n");
            assembly.push_str("\tmov rsp, rbp\n");
            assembly.push_str("\tpop rbp\n");
            assembly.push_str("\tret\n");
        }
        Token::Operand(op) => {
            // '='の時は特別に、左辺値の扱いが他の二項演算と異なる。
            if op == "=" {
                if let Some(b) = &node.lhs {
                    generate_lvar(b, vars, assembly);
                }
                if let Some(b) = &node.rhs {
                    print_assembly_by_node(b, vars, assembly);
                }
                assembly.push_str("\tpop rdi\n");
                assembly.push_str("\tpop rax\n");
                assembly.push_str("\tmov [rax], rdi\n");
                assembly.push_str("\tpush rdi\n");
                return;
            }
            // operand ... 二項
            if let Some(b) = &node.lhs {
                print_assembly_by_node(b, vars, assembly);
            }
            if let Some(b) = &node.rhs {
                print_assembly_by_node(b, vars, assembly);
            }
            assembly.push_str("\tpop rdi\n");
            assembly.push_str("\tpop rax\n");
            match op {
                "+" => {
                    assembly.push_str("\tadd rax, rdi\n");
                    assembly.push_str("\tpush rax\n");
                }
                "-" => {
                    assembly.push_str("\tsub rax, rdi\n");
                    assembly.push_str("\tpush rax\n");
                }
                "*" => {
                    assembly.push_str("\timul rax, rdi\n");
                    assembly.push_str("\tpush rax\n");
                }
                "/" => {
                    assembly.push_str("\tcqo\n");
                    assembly.push_str("\tidiv rdi\n");
                    assembly.push_str("\tpush rax\n");
                }
                "<" => {
                    assembly.push_str("\tcmp rax, rdi\n");
                    assembly.push_str("\tsetl al\n");
                    assembly.push_str("\tmovzb rax, al\n");
                    assembly.push_str("\tpush rax\n");
                }
                "<=" => {
                    assembly.push_str("\tcmp rax, rdi\n");
                    assembly.push_str("\tsetle al\n");
                    assembly.push_str("\tmovzb rax, al\n");
                    assembly.push_str("\tpush rax\n");
                }
                "!=" => {
                    assembly.push_str("\tcmp rax, rdi\n");
                    assembly.push_str("\tsetne al\n");
                    assembly.push_str("\tmovzb rax, al\n");
                    assembly.push_str("\tpush rax\n");
                }
                "==" => {
                    assembly.push_str("\tcmp rax, rdi\n");
                    assembly.push_str("\tsete al\n");
                    assembly.push_str("\tmovzb rax, al\n");
                    assembly.push_str("\tpush rax\n");
                }
                _ => {
                    //
                }
            }
        }
        Token::Reserved(_name) => {
            // 未実装
        }
    }
}
