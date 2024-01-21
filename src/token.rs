#[derive(PartialEq)]
pub enum TokenType {
    ILLEGAL,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
