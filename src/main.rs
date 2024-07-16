pub mod parser;
pub mod interpreter;

use std::{env, process};
use parser::{Token, parse};
use interpreter::{shunt, interpret};

/// generic error function
fn error(msg: &str) {
    println!("boxi: {}", msg);
    println!("Try 'boxi --help' for more information.");
    println!();
    process::exit(1);
}

/// prints usage information
fn usage() {
    let usage = "boxi 0.1.0
A simple calculator written in Rust.

USAGE: boxi [expression]

EXAMPLES:
    boxi '1 + 2'
    boxi '0x3 * 0x4'
    boxi '5 / 6'
    boxi '7 - 8'
    ";
    println!("{}", usage);
}

/// pretty prints number in all bases
fn print_num(num: i64) {
    println!("decimal: {}", num);
    println!("hexadecimal: 0x{:x}", num);
    println!("octal: 0o{:o}", num);
    println!("binary: 0b{:b}", num);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { error("invalid number of arguments"); }
    if args[1] == "--help" { usage(); process::exit(0); }
    let input = args[1].replace(" ", "");
    let tokens: Result<Vec<Token>, &str> = parse(&input);
    if tokens.is_err() { error(tokens.as_ref().err().unwrap()); }
    /*
    for token in tokens.clone().unwrap().iter() {
        println!("normal: {:?}", token);
    }
    */
    let shunted: Vec<Token> = shunt(tokens.unwrap());
    for token in shunted.iter() {
        println!("{:?}", token);
    }
    let result = interpret(shunted);
    print_num(result);
}
