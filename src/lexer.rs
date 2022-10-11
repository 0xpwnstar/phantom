use crate::token::Token;

pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: usize,
    pub read_position: usize,
}


impl<'a> Iterator for Lexer<'a>{
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.position = self.read_position;
        // if self.position >= self.input.len() {
        //     return None
        // }else if self.input[self.read_position..self.read_position+1].chars().all(|c| {c.is_alphabetic()}) {
        //     loop {
        //         if self.read_position+1 ==self.input.len(){
        //             self.read_position += 1; 
        //             return Some(&self.input[self.position..])
        //         }
        //         let i = &self.input[self.read_position..self.read_position+1];
        //         if i.chars().all(|c| {c.is_alphabetic()}) {
        //             self.read_position += 1;
        //         }else {
        //             break
        //         }
    
        //     }
        //     return Some(&self.input[self.position..self.read_position])            
        // }else if &self.input[self.read_position..self.read_position+1] == " "{
        //     self.read_position += 1;
        //     return self.next()
        // // }else if &self.input[self.read_position..self.read_position+1] == "="{
        // //     if  &self.input[self.read_position..self.read_position+2] == "=="{
        // //         self.read_position += 2;
        // //         return Some("==")
        // //     }
        // //     self.read_position += 1;
        // //     return Some("=")
        // }else if self.input[self.read_position..self.read_position+1].parse::<f64>().is_ok() {
        //     loop {
        //         if self.input[self.read_position..self.read_position+1].parse::<f64>().is_ok() {
        //             self.read_position += 1;
        //         }else {   
        //             break
        //         }    
        //     }
        //     return Some(&self.input[self.position..self.read_position])
        // }else if &self.input[self.read_position..self.read_position+1] == "!"{
            // if  &self.input[self.read_position..self.read_position+2] == "!="{
            //     self.read_position += 2;
            //     return Some("!=")
            // }
            // self.read_position += 1;
            // return Some("!")

        // }else if &self.input[self.read_position..self.read_position+1] == "("{
        //     self.read_position += 1;
        //     return Some("(")
        // }
        // else if &self.input[self.read_position..self.read_position+1] == ")"{
        //     self.read_position += 1;
        //     return Some(")")
        // }else if &self.input[self.read_position..self.read_position+1] == "?"{
        //     self.read_position += 1;
        //     return Some("?")
        // }else if &self.input[self.read_position..self.read_position+1] == "!"{
        //     self.read_position += 1;
        //     return Some(")")
        // }else if &self.input[self.read_position..self.read_position+1] == "}"{
        //     self.read_position += 1;
        //     return Some("}")
        // }else if &self.input[self.read_position..self.read_position+1] == "{"{
        //     self.read_position += 1;
        //     return Some("{")
        // }else if &self.input[self.read_position..self.read_position+1] == "]"{
        //     self.read_position += 1;
        //     return Some("]")
        // }else if &self.input[self.read_position..self.read_position+1] == "["{
        //     self.read_position += 1;
        //     return Some("[")
        // }else if &self.input[self.read_position..self.read_position+1] == "+"{
        //     self.read_position += 1;
        //     return Some("+")
        // }else if &self.input[self.read_position..self.read_position+1] == "-"{
        //     self.read_position += 1;
        //     return Some("-")
        // }else if &self.input[self.read_position..self.read_position+1] == ","{
        //     self.read_position += 1;
        //     return Some(",")
        // }else if &self.input[self.read_position..self.read_position+1] == ";"{
        //     self.read_position += 1;
        //     return Some(";")
        // }else if &self.input[self.read_position..self.read_position+1] == "<"{
        //     self.read_position += 1;
        //     return Some("<")
        // }else if &self.input[self.read_position..self.read_position+1] == ">"{
        //     self.read_position += 1;
        //     return Some(">")
        // }else if &self.input[self.read_position..self.read_position+1] == "*"{
        //     self.read_position += 1;
        //     return Some("*")
        // }
        if self.read_position < self.input.len(){
            let m = &self.input[self.position..self.read_position+1];
            match m {

                //case =
                "=" => {
                    if  &self.input[self.read_position..self.read_position+2] == "=="{
                        self.read_position += 2;
                        return Some(Token{
                            lexeme: &self.input[self.position..self.read_position],
                            token: crate::token::TokenType::EQUAL}
                        );
                        
                    }
                    self.read_position += 1;
                    return Some(Token{lexeme: m,token: crate::token::TokenType::ASSIGN})
                                   
                }

                //case " "
                " " => {
                    self.read_position += 1;
                    return self.next()
                }

                //case !
                "!" => {
                    if  &self.input[self.read_position..self.read_position+2] == "!="{
                        self.read_position += 2;
                        return Some(Token{
                            lexeme: &self.input[self.position..self.read_position],
                            token: crate::token::TokenType::EQUAL}
                        );
                    }
                    self.read_position += 1;
                    return Some(Token{
                        lexeme: &self.input[self.position..self.read_position],
                        token: crate::token::TokenType::EQUAL}
                    );
                }

                _ => {
                    return None
                }
            }
        };
        return None

    }
}



#[cfg(test)]
mod tests {

    mod lexer {
        use crate::lexer::Lexer;



        #[test]
        fn lexer() {
            let mut l = Lexer{input:"   !    ==",position:3,read_position:0};
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());
            print!("{:?}",l.next());

        } 
    }
}