use std::io::{self, BufRead};

use crate::lexer::Lexer;

const PROMPT:&str = ">>";


pub fn start(input: io::Stdin){
    let mut iterator = input.lock().lines();
    
    loop {
        println!("{}",PROMPT);
        if let Ok(next) = iterator.next().unwrap(){
            let l = Lexer{input: &next,position: 0, read_position: 0};
            for i in l {
                println!("{:?}",i);
                if next == String::from("quit"){
                    return
                }
            }
        }
        else {
            return
        }
    }
     
}