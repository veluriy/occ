use std::{env, process};
pub mod lex;
use crate::lex::{tokenize};
pub mod parser;
use crate::parser::{Parser};

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
    let mut parser = Parser {
        tokenizer: & mut iter
    };
    parser.parse();

    println!("  pop rax");
    println!("  ret");
}

fn main() {
    tree()
}
