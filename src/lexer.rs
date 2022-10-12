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
        

        if self.position >= self.input.len() {
            return None
        }else if self.input[self.read_position..self.read_position+1].chars().all(|c| {c.is_alphabetic()}) {
            //when we have input as letter `x` we could have a letter ahead of x
            //check possible cases and return the word as Ident, function ....
            loop {
                if self.read_position+1 ==self.input.len(){
                    self.read_position += 1; 
                    return Some(
                        Token { lexeme: &self.input[self.position..], token: crate::token::TokenType::EOF })
                }
                let i = &self.input[self.read_position..self.read_position+1];
                if i.chars().all(|c| {c.is_alphabetic()}) {
                    self.read_position += 1;
                }else {
                    break
                }
    
            }
            return Some(
                Token { 
                    lexeme: &self.input[self.position..self.read_position], 
                    token: crate::token::TokenType::IDENT
                }
            )            
        }
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
                            token: crate::token::TokenType::NOTEQUAL}
                        );
                    }
                    self.read_position += 1;
                    return Some(Token{
                        lexeme: &self.input[self.position..self.read_position],
                        token: crate::token::TokenType::NOT}
                    );
                }

                "(" => {
                    self.read_position += 1;
                    return Some(Token{
                        lexeme: &self.input[self.position..self.read_position],
                        token: crate::token::TokenType::LPAREN}
                    );
                }
                ")" => {
                    self.read_position += 1;
                    return Some(Token{
                        lexeme: &self.input[self.position..self.read_position],
                        token: crate::token::TokenType::RPAREN}
                    );
                }
                "}" => {
                    self.read_position += 1;
                    return Some(Token{
                        lexeme: &self.input[self.position..self.read_position],
                        token: crate::token::TokenType::RBRACE}
                    );
                }
                "{" => {
                    self.read_position += 1;
                    return Some(Token{
                        lexeme: &self.input[self.position..self.read_position],
                        token: crate::token::TokenType::LBRACE}
                    );
                }
                "+" => {
                    self.read_position += 1;
                    return Some(Token{
                        lexeme: &self.input[self.position..self.read_position],
                        token: crate::token::TokenType::PLUS}
                    );
                }
                "-" => {
                    self.read_position += 1;
                    return Some(Token{
                        lexeme: &self.input[self.position..self.read_position],
                        token: crate::token::TokenType::MINUS}
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
            let mut l = Lexer{input:" ddv = sdsds {}()+-   !    ==",position:3,read_position:0};
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