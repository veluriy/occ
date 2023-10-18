pub mod generator;
pub mod lex;
pub mod parser;
pub mod types;

use std::collections::HashMap;

use crate::generator::*;
use crate::types::{tokenize, Parser, Variables};

pub fn generate_assembly(str: &str) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    let mut iter = tokenize(str);
    let mut vars = Variables {
        offsets: &mut HashMap::new()
    };
    let mut parser = Parser {
        token_iter: &mut iter,
        vars: &mut vars,
    };
    let node = parser.parse();
    /*if let Some(b) = node {
        print_assembly_by_node(&b);
    }*/
    println!("  pop rax");
    println!("  ret");
}
