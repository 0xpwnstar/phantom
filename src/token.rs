
#[derive(Debug)]
pub struct Token<'a>{
    pub lexeme: &'a str,
    pub token: TokenType
}

#[derive(Debug)]
pub enum TokenType {
    EOF,
    ILLEGAL,

    IDENT,
    INT,

    EQUAL,
    NOTEQUAL,
    NOT,

    ASSIGN,
    PLUS,
    MINUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET    
}