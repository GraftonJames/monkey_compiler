#[derive(PartialEq)]
pub enum TokenType {
    ILLEGAL,
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,
    EQ,
    NOTEQ,

    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[macro_export]
macro_rules! token {
    ($tt: expr, $l:expr) => {{
        Token {
            token_type: $tt,
            literal: $l,
        }
    }};
}

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
