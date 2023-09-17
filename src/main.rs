use std::{env, process};
pub mod tokenizer;
use crate::tokenizer::{tokenize, Token};
fn _prev_main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() != 2 {
        eprintln!("引数の数が異なります");
        process::exit(1);
    }
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", &args[1]);
    println!("  ret");
}

fn main(){
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の数が異なります");
        process::exit(1);
    }
    let mut iter = tokenize(&args[1]);

    println!("  mov rax, {}", &iter.next().unwrap().expect_num());
    while let Some(token) = iter.next() {
        let number = iter.next().unwrap().expect_num();
        match token {
            Token::Plus => println!("  add rax, {}", number),
            Token::Minus => println!("  sub rax, {}", number),
            _ => panic!("")
        };
    }
    // println!("  mov rax, {}", &args[1]);
    println!("  ret");
}
