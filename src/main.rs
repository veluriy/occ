use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() != 2 {
        eprintln!("引数の数が異なります");
        std::process::exit(1);
    }
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", &args[1]);
    println!("  ret");
}
