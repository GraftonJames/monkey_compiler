use std::ops::Deref;

use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}
fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
fn lookup_ident(ident: &String) -> TokenType {
    match ident.deref() {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        _ => TokenType::IDENT,
    }
}
impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }
    // @MARK iterating through chars each time is cumbersome especailly when chars() is allready an
    // iterator;
    fn read_char(&mut self) {
        self.ch = self.input.chars().take(self.read_position).next();
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn read_identifier(&mut self) -> String {
        let ch = self.ch.unwrap();
        let end_string_slice = self.position;
        if is_letter(ch) {
            self.read_char();
        }
        self.input[self.position..end_string_slice].to_string()
    }
    fn read_single_char_token(&mut self) -> Option<Token> {
        let ch = self.ch.unwrap();
        match ch {
            '=' => Some(Token {
                token_type: TokenType::ASSIGN,
                literal: ch.to_string(),
            }),
            ';' => Some(Token {
                token_type: TokenType::SEMICOLON,
                literal: ch.to_string(),
            }),
            '(' => Some(Token {
                token_type: TokenType::LPAREN,
                literal: ch.to_string(),
            }),
            ')' => Some(Token {
                token_type: TokenType::RPAREN,
                literal: ch.to_string(),
            }),
            ',' => Some(Token {
                token_type: TokenType::COMMA,
                literal: ch.to_string(),
            }),
            '+' => Some(Token {
                token_type: TokenType::PLUS,
                literal: ch.to_string(),
            }),
            '{' => Some(Token {
                token_type: TokenType::LBRACE,
                literal: ch.to_string(),
            }),
            '}' => Some(Token {
                token_type: TokenType::RBRACE,
                literal: ch.to_string(),
            }),
            _ => None,
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok;
        if self.ch.is_none() {
            tok = None;
        } else if let Some(token) = self.read_single_char_token() {
            tok = Some(token);
        } else if is_letter(self.ch.unwrap()) {
            let ident = self.read_identifier();
            tok = Some(Token {
                token_type: lookup_ident(&ident),
                literal: ident,
            });
        } else {
            tok = Some(Token {
                token_type: TokenType::ILLEGAL,
                literal: self.ch.unwrap().to_string(),
            });
        };
        self.read_char();
        tok
    }
}
