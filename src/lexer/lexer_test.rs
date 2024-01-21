use crate::lexer::Lexer;
use crate::token;
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
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::PLUS, String::from("+")),
            token!(TokenType::LPAREN, String::from("(")),
            token!(TokenType::RPAREN, String::from(")")),
            token!(TokenType::LBRACE,String::from("{")),
            token!(TokenType::RBRACE, String::from("}")),
            token!(TokenType::COMMA, String::from(",")),
            token!(TokenType::SEMICOLON, String::from(";")),
        ].into_iter()
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
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("five")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::INT, String::from("5")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("ten")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::INT, String::from("10")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("add")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::FUNCTION, String::from("fn")),
            token!(TokenType::LPAREN, String::from("(")),
            token!(TokenType::IDENT, String::from("x")),
            token!(TokenType::COMMA, String::from(",")),
            token!(TokenType::IDENT, String::from("y")),
            token!(TokenType::RPAREN, String::from(")")),
            token!(TokenType::LBRACE, String::from("{")),
            token!(TokenType::IDENT, String::from("x")),
            token!(TokenType::PLUS, String::from("+")),
            token!(TokenType::IDENT, String::from("y")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::RBRACE, String::from("}")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("result")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::IDENT, String::from("add")),
            token!(TokenType::LPAREN, String::from("(")),
            token!(TokenType::IDENT, String::from("five")),
            token!(TokenType::COMMA, String::from(",")),
            token!(TokenType::IDENT, String::from("ten")),
            token!(TokenType::RPAREN, String::from(")")),
            token!(TokenType::SEMICOLON, String::from(";")),
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
        !-/*5
        5 < 10 > 5
        ",
        vec![
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("five")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::INT, String::from("5")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("ten")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::INT, String::from("10")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("add")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::FUNCTION, String::from("fn")),
            token!(TokenType::LPAREN, String::from("(")),
            token!(TokenType::IDENT, String::from("x")),
            token!(TokenType::COMMA, String::from(",")),
            token!(TokenType::IDENT, String::from("y")),
            token!(TokenType::RPAREN, String::from(")")),
            token!(TokenType::LBRACE, String::from("{")),
            token!(TokenType::IDENT, String::from("x")),
            token!(TokenType::PLUS, String::from("+")),
            token!(TokenType::IDENT, String::from("y")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::RBRACE, String::from("}")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("result")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::IDENT, String::from("add")),
            token!(TokenType::LPAREN, String::from("(")),
            token!(TokenType::IDENT, String::from("five")),
            token!(TokenType::COMMA, String::from(",")),
            token!(TokenType::IDENT, String::from("ten")),
            token!(TokenType::RPAREN, String::from(")")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::BANG, String::from("!")),
            token!(TokenType::MINUS, String::from("-")),
            token!(TokenType::SLASH, String::from("/")),
            token!(TokenType::ASTERISK, String::from("*")),
            token!(TokenType::INT, String::from("5")),
            token!(TokenType::INT, String::from("5")),
            token!(TokenType::LT, String::from("<")),
            token!(TokenType::INT, String::from("10")),
            token!(TokenType::GT, String::from(">")),
            token!(TokenType::INT, String::from("5")),
        ].into_iter()
    ),
    next_test_keywords: (
        "
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
        ",
        vec![
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("five")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::INT, String::from("5")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("ten")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::INT, String::from("10")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("add")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::FUNCTION, String::from("fn")),
            token!(TokenType::LPAREN, String::from("(")),
            token!(TokenType::IDENT, String::from("x")),
            token!(TokenType::COMMA, String::from(",")),
            token!(TokenType::IDENT, String::from("y")),
            token!(TokenType::RPAREN, String::from(")")),
            token!(TokenType::LBRACE, String::from("{")),
            token!(TokenType::IDENT, String::from("x")),
            token!(TokenType::PLUS, String::from("+")),
            token!(TokenType::IDENT, String::from("y")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::RBRACE, String::from("}")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::LET, String::from("let")),
            token!(TokenType::IDENT, String::from("result")),
            token!(TokenType::ASSIGN, String::from("=")),
            token!(TokenType::IDENT, String::from("add")),
            token!(TokenType::LPAREN, String::from("(")),
            token!(TokenType::IDENT, String::from("five")),
            token!(TokenType::COMMA, String::from(",")),
            token!(TokenType::IDENT, String::from("ten")),
            token!(TokenType::RPAREN, String::from(")")),
            token!(TokenType::SEMICOLON, String::from(";")),
            token!(TokenType::BANG, String::from("!")),
            token!(TokenType::MINUS, String::from("-")),
            token!(TokenType::SLASH, String::from("/")),
            token!(TokenType::ASTERISK, String::from("*")),
            token!(TokenType::INT, String::from("5")),
            token!(TokenType::INT, String::from("5")),
            token!(TokenType::LT, String::from("<")),
            token!(TokenType::INT, String::from("10")),
            token!(TokenType::GT, String::from(">")),
            token!(TokenType::INT, String::from("5")),
        ].into_iter()
    ),
}
