use std::io;

pub mod lexer;
pub mod token;
pub mod repl;

fn main() -> std::io::Result<()>{
    println!("Hello! This is phantom programming language!");
    return repl::start(io::stdin(), io::stdout())
}
