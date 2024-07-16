pub mod parser;
pub mod interpreter;

use std::{env, process};
use parser::{Token, parse};
use interpreter::shunt;

fn error(msg: &str) {
    println!("boxi: {}", msg);
    println!("Try 'boxi --help' for more information.");
    println!();
    process::exit(1);
}

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { error("invalid number of arguments"); }
    if args[1] == "--help" { usage(); process::exit(0); }
    let tokens: Result<Vec<Token>, &str> = parse(&args[1]);
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
}
