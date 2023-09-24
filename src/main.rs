use std::{env, process};
pub mod parser;
use crate::parser::{tokenize, Token};

// 構文木を利用して四則演算を行う
fn tree() {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の数が異なります");
        process::exit(1);
    }
    let mut iter = tokenize(&args[1]);
    // 式に対応した構文木を取得
    let node = iter.expr();
    // パターンに拘束してtokenize
    if let Some(b) = node {
        parser::gen(&b);
    }
    println!("  pop rax");
    println!("  ret");
}

fn main() {
    tree()
}
