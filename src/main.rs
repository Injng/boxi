use std::env;

pub mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        println!("{}", arg);
    }
}
