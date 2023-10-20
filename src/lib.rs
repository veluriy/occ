pub mod generator;
pub mod lex;
pub mod parser;
pub mod types;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

use crate::generator::*;
use crate::types::{tokenize, Parser, Variables};

pub struct Input {
    input_file_name: String,
    output_file_name: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let input_file_name = args[1].clone();
        let output_file_name = args[2].clone();

        Ok(Input {
            input_file_name,
            output_file_name,
        })
    }
}

pub fn run(input: Input) {
    let mut input_file = match File::open(input.input_file_name) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    let mut output_file = match File::create(input.output_file_name) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    let mut contents = String::new();
    match input_file.read_to_string(&mut contents) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    let assembly = generate_assembly(&contents);
    match write!(output_file, "{}", assembly) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    match output_file.flush() {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
}

fn generate_assembly(str: &str) -> String {
    let mut assembly = String::new();

    assembly.push_str(".intel_syntax noprefix\n");
    assembly.push_str(".globl main\n");
    assembly.push_str("main:\n");
    let mut iter = tokenize(str);
    let mut vars = Variables {
        offsets: &mut HashMap::new(),
    };
    let mut parser = Parser {
        token_iter: &mut iter,
        vars: &mut vars,
    };
    let nodes = parser.parse();
    assembly.push_str("\tpush rbp\n");
    assembly.push_str("\tmov rbp, rsp\n");
    assembly.push_str(&format!("\tsub rsp, {}\n", &parser.vars.offsets.len() * 8));

    for node in nodes {
        print_assembly_by_node(&node, parser.vars, &mut assembly);
        assembly.push_str("\tpop rax\n");
    }
    assembly.push_str("\tmov rsp, rbp\n");
    assembly.push_str("\tpop rbp\n");
    assembly.push_str("\tret\n");

    assembly
}
