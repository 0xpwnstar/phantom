use std::io::{self, BufRead, Write};

use crate::lexer::Lexer;

const PROMPT:&str = ">>";

pub fn start(input: io::Stdin,output: io::Stdout) -> std::io::Result<()>{
    let mut iterator = input.lock().lines();
    
    loop {
        println!("{}",PROMPT);
        if let Ok(next) = iterator.next().unwrap(){
            let l = Lexer{input: &next,position: 0, read_position: 0};
            for i in l {

                output.lock().write_fmt(format_args!("{:?} \n",i))?;

                if next == String::from("quit"){
                    return Result::Ok(())
                }
            }
        }
        else {
            return Result::Ok(())
        }
    }
     
}