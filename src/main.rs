use burn;
use std::env;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Expected one filename argument");
        std::process::exit(1);
    }
    burn::compile_file(&args[0]); // will panic if it cannot compile
    println!("[+] All checks passed");
}
