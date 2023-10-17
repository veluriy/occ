use occ::generate_assembly;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の数が異なります");
        process::exit(1);
    }
    // args[1]から空白を削除してそれをgenerate_assembly関数に渡したい
    //args[1].retain(|c| !c.is_whitespace());
    //
    generate_assembly(&args[1]);
}
