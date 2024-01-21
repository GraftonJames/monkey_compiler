use crate::lexer::Lexer;
use crate::token::*;

macro_rules! lexer_test_next_token {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert!(Lexer::new(input).zip(expected).fold(true,|acc, (res,exp)| acc && (res == exp)));
            }

        )*
    };
}
lexer_test_next_token! {
    test_next_single_char_tokens: (
        "=+(){},;",
        vec![
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: String::from("+"),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
        ]
        .into_iter()
    ),
    next_test_basic_code: (
        "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        ",
        vec![
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: String::from("fn"),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: String::from("+"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("result"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
        ].into_iter()
    ),
    next_test_gibberish_code: (
        "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        ",
        vec![
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::INT,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: String::from("fn"),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: String::from("+"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("result"),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: String::from(";"),
            },
        ]
        .into_iter()
    ),
}
