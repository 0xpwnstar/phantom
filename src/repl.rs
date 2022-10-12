use std::io::{self, BufRead};

const PROMPT:&str = ">>";


pub fn start(input: io::Stdin){
    let mut iterator = input.lock().lines();
    
    loop {
        println!("{}",PROMPT);
        if let Ok(next) = iterator.next().unwrap(){
            if next == String::from("quit"){
                return
            }
            println!("{:?}",next);
        }
        else {
            return
        }
    }
     
}