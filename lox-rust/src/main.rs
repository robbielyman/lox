use std::{env, process, io, fs::File, io::Read, path::Path};

mod lox;
use lox::Lox;
mod token;

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => println!("Usage: rlox [script]")
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut src = String::new();
    let mut lox = Lox::new();

    loop {
        let len = stdin.read_line(&mut src).expect("Bad file input");
        if len == 0 { break; }
        lox.run(&src);
        src.clear();
        lox.reset_error();
    }
}

fn run_file(file_name: &str) {
    let mut lox = Lox::new();
    let path = Path::new(file_name);
    let mut file = File::open(path).expect("Usage: rlox [script]");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("Bad file content");
    lox.run(&src);
    if lox.had_error() { process::exit(65); }
}
