pub mod generator;
pub mod lex;
pub mod parser;
pub mod types;

use crate::generator::*;
use crate::types::{tokenize, Parser};

pub fn generate_assembly(str: &str) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    let mut iter = tokenize(str);
    let mut parser = Parser {
        tokenizer: &mut iter,
    };
    let node = parser.parse();
    if let Some(b) = node {
        print_assembly_by_node(&b);
    }
    println!("  pop rax");
    println!("  ret");
}
