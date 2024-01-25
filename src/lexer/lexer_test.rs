use crate::lexer::Lexer;
use crate::token;
use crate::token::*;

macro_rules! lexer_test_next_token {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, mut expected) = $value;
                for token in Lexer::new(input) {
                    assert_eq!(Some(token), expected.next());
                }
            }

        )*
    };
}
lexer_test_next_token! {
    test_next_single_char_tokens: (
        String::from("=+(){},;"),
        vec![
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Plus, String::from("+")),
            token!(TokenType::Lparen, String::from("(")),
            token!(TokenType::Rparen, String::from(")")),
            token!(TokenType::Lbrace,String::from("{")),
            token!(TokenType::Rbrace, String::from("}")),
            token!(TokenType::Comma, String::from(",")),
            token!(TokenType::Semicolon, String::from(";")),
        ].into_iter()
    ),
    next_test_basic_code: (
        String::from("
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        "),
        vec![
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("five")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Int, String::from("5")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("ten")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Int, String::from("10")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("add")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Function, String::from("fn")),
            token!(TokenType::Lparen, String::from("(")),
            token!(TokenType::Ident, String::from("x")),
            token!(TokenType::Comma, String::from(",")),
            token!(TokenType::Ident, String::from("y")),
            token!(TokenType::Rparen, String::from(")")),
            token!(TokenType::Lbrace, String::from("{")),
            token!(TokenType::Ident, String::from("x")),
            token!(TokenType::Plus, String::from("+")),
            token!(TokenType::Ident, String::from("y")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Rbrace, String::from("}")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("result")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Ident, String::from("add")),
            token!(TokenType::Lparen, String::from("(")),
            token!(TokenType::Ident, String::from("five")),
            token!(TokenType::Comma, String::from(",")),
            token!(TokenType::Ident, String::from("ten")),
            token!(TokenType::Rparen, String::from(")")),
            token!(TokenType::Semicolon, String::from(";")),
        ].into_iter()
    ),
    next_test_gibberish_code: (
        String::from("
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5
        5 < 10 > 5
        "),
        vec![
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("five")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Int, String::from("5")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("ten")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Int, String::from("10")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("add")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Function, String::from("fn")),
            token!(TokenType::Lparen, String::from("(")),
            token!(TokenType::Ident, String::from("x")),
            token!(TokenType::Comma, String::from(",")),
            token!(TokenType::Ident, String::from("y")),
            token!(TokenType::Rparen, String::from(")")),
            token!(TokenType::Lbrace, String::from("{")),
            token!(TokenType::Ident, String::from("x")),
            token!(TokenType::Plus, String::from("+")),
            token!(TokenType::Ident, String::from("y")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Rbrace, String::from("}")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("result")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Ident, String::from("add")),
            token!(TokenType::Lparen, String::from("(")),
            token!(TokenType::Ident, String::from("five")),
            token!(TokenType::Comma, String::from(",")),
            token!(TokenType::Ident, String::from("ten")),
            token!(TokenType::Rparen, String::from(")")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Bang, String::from("!")),
            token!(TokenType::Minus, String::from("-")),
            token!(TokenType::Slash, String::from("/")),
            token!(TokenType::Asterisk, String::from("*")),
            token!(TokenType::Int, String::from("5")),
            token!(TokenType::Int, String::from("5")),
            token!(TokenType::Lt, String::from("<")),
            token!(TokenType::Int, String::from("10")),
            token!(TokenType::Gt, String::from(">")),
            token!(TokenType::Int, String::from("5")),
        ].into_iter()
    ),
    next_test_keywords: (
        String::from("
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5
        5 < 10 > 5

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 != 9
        10 == 10
        "),
        vec![
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("five")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Int, String::from("5")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("ten")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Int, String::from("10")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("add")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Function, String::from("fn")),
            token!(TokenType::Lparen, String::from("(")),
            token!(TokenType::Ident, String::from("x")),
            token!(TokenType::Comma, String::from(",")),
            token!(TokenType::Ident, String::from("y")),
            token!(TokenType::Rparen, String::from(")")),
            token!(TokenType::Lbrace, String::from("{")),
            token!(TokenType::Ident, String::from("x")),
            token!(TokenType::Plus, String::from("+")),
            token!(TokenType::Ident, String::from("y")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Rbrace, String::from("}")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Let, String::from("let")),
            token!(TokenType::Ident, String::from("result")),
            token!(TokenType::Assign, String::from("=")),
            token!(TokenType::Ident, String::from("add")),
            token!(TokenType::Lparen, String::from("(")),
            token!(TokenType::Ident, String::from("five")),
            token!(TokenType::Comma, String::from(",")),
            token!(TokenType::Ident, String::from("ten")),
            token!(TokenType::Rparen, String::from(")")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Bang, String::from("!")),
            token!(TokenType::Minus, String::from("-")),
            token!(TokenType::Slash, String::from("/")),
            token!(TokenType::Asterisk, String::from("*")),
            token!(TokenType::Int, String::from("5")),
            token!(TokenType::Int, String::from("5")),
            token!(TokenType::Lt, String::from("<")),
            token!(TokenType::Int, String::from("10")),
            token!(TokenType::Gt, String::from(">")),
            token!(TokenType::Int, String::from("5")),

            token!(TokenType::If, String::from("if")),
            token!(TokenType::Lparen, String::from("(")),
            token!(TokenType::Int, String::from("5")),
            token!(TokenType::Lt, String::from("<")),
            token!(TokenType::Int, String::from("10")),
            token!(TokenType::Rparen, String::from(")")),
            token!(TokenType::Lbrace, String::from("{")),
            token!(TokenType::Return, String::from("return")),
            token!(TokenType::True, String::from("true")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Rbrace, String::from("}")),
            token!(TokenType::Else, String::from("else")),
            token!(TokenType::Lbrace, String::from("{")),
            token!(TokenType::Return, String::from("return")),
            token!(TokenType::False, String::from("false")),
            token!(TokenType::Semicolon, String::from(";")),
            token!(TokenType::Rbrace, String::from("}")),
            token!(TokenType::Int, String::from("10")),
            token!(TokenType::Noteq, String::from("!=")),
            token!(TokenType::Int, String::from("9")),
            token!(TokenType::Int, String::from("10")),
            token!(TokenType::Eq, String::from("==")),
            token!(TokenType::Int, String::from("10")),
        ].into_iter()
    ),
}
