mod token;
mod scanner;
mod parser;
mod expression;
mod interpreter;
mod statement;

use std::env;
use std::io;
use std::io::{Read, Write};
use std::fs::File;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

fn run_file(path: &str) {
    let mut contents = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut contents).unwrap();
    run(&contents).unwrap();
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" || input == "exit" { break; }
        run(input).unwrap();
    }
}

fn run(src: &str) -> Result<(), String> {
    let scanner = Scanner::new(src.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expressions = parser.parse();
    // for expr in expressions {
    //     println!("{:?}\n", expr);
    // }
    let interpreter = Interpreter::new();
    interpreter.interpret(expressions);
    Ok(())
}

fn main() {
    if env::args().len() > 2 {
        eprintln!("Usage: {} [script]", env::args().next().unwrap());
    } else if env::args().len() == 2 {
        run_file(&env::args().nth(1).unwrap());
    } else {
        run_prompt();
    }
}
