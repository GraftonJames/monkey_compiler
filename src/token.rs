#[derive(PartialEq, Debug)]
pub enum TokenType {
        Illegal,
        Ident,
        Int,

        // Operators
        Assign,
        Plus,
        Minus,
        Bang,
        Asterisk,
        Slash,

        Lt,
        Gt,
        Eq,
        Noteq,

        Comma,
        Semicolon,
        Lparen,
        Rparen,
        Lbrace,
        Rbrace,

        // Keywords
        Function,
        Let,
        True,
        False,
        If,
        Else,
        Return,
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

#[derive(PartialEq, Debug)]
pub struct Token {
        pub token_type: TokenType,
        pub literal: String,
}
