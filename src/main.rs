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
    print!("\x1B[1mdec:\x1B[0m {} \t", num);
    print!("\x1B[1mhex:\x1B[0m 0x{:x} \t", num);
    print!("\x1B[1moct:\x1B[0m 0o{:o} \t", num);
    print!("\x1B[1mbin:\x1B[0m 0b{:b} \t", num);
}

fn main() {
    // handle command line arguments
    let args: Vec<String> = env::args().collect();
    let verbose = args.len() > 2 && args[2] == "--verbose";
    if args.len() < 2 { error("invalid number of arguments"); }
    if args[1] == "--help" { usage(); process::exit(0); }

    let input = args[1].replace(" ", "");
    let tokens: Result<Vec<Token>, &str> = parse(&input);
    if tokens.is_err() { error(tokens.as_ref().err().unwrap()); }

    // print infix tokens if verbose
    if verbose {
        println!("--- infix tokens ---");
        for token in tokens.clone().unwrap().iter() {
            println!("{:?}", token);
        }
        println!();
    }

    let shunted: Vec<Token> = shunt(tokens.unwrap());

    // print postfix tokens if verbose
    if verbose {
        println!("--- postfix tokens ---");
        for token in shunted.iter() {
            println!("{:?}", token);
        }
        println!();
    }

    let result = interpret(shunted);
    print_num(result);
}
