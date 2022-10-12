use std::io;

pub mod lexer;
pub mod token;
pub mod repl;

fn main() {
    println!("Hello! This is phantom programming language!");
    repl::start(io::stdin());
}
