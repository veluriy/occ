use std::{env, process};
use occ::generate_assembly;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の数が異なります");
        process::exit(1);
    }
    generate_assembly(&args[1]);
}
