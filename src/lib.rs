pub mod lex;
pub mod parser;

use crate::lex::tokenize;
use crate::parser::Parser;

pub fn generate_assembly(str: &str) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    let mut iter = tokenize(str);
    let mut parser = Parser {
        tokenizer: &mut iter,
    };
    parser.parse();
    println!("  pop rax");
    println!("  ret");
}