use std::io::{self, BufRead, Read};
use std::env;
use std::fs::File;

mod errors;
mod scanner;
mod tokens;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        };
    } else {
        match run_prompt() {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }
}

fn run_file(file: &str) -> std::io::Result<()> {
    let mut fd = File::open(file)?;
    let mut program = String::new();
    fd.read_to_string(&mut program)?;
    run(&program);
    Ok(())
}

fn run_prompt() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        stdin.lock().read_line(&mut input)?;
        run(&input);
        input.clear();
    }
}

fn run(program: &str) {
    println!("The program is: {}", program);
}
