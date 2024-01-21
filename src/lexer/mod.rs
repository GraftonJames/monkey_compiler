#[cfg(test)]
mod lexer_test;

use std::{ops::Deref, str::Chars};

use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: Chars<'a>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}
fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
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
            input: input.chars(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }
    fn read_char(&mut self) {
        self.ch = self.input.next();
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn read_identifier(&mut self) -> String {
        let mut ident: Vec<char> = vec![];
        while self.ch.is_some() && is_letter(self.ch.unwrap()) {
            ident.push(self.ch.unwrap());
            self.read_char();
        }
        ident.into_iter().collect()
    }
    fn read_number(&mut self) -> String {
        let mut int = vec![];
        while self.ch.is_some() && self.ch.unwrap().is_digit(10) {
            int.push(self.ch.unwrap());
            self.read_char();
        }
        int.into_iter().collect()
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

    fn skip_whitespace(&mut self) {
        while self.ch == Some(' ')
            || self.ch == Some('\t')
            || self.ch == Some('\n')
            || self.ch == Some('\r')
        {
            self.read_char();
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok;

        self.skip_whitespace();

        if self.ch.is_none() {
            tok = None;
            self.read_char();
        } else if let Some(token) = self.read_single_char_token() {
            tok = Some(token);
            self.read_char();
        } else if is_letter(self.ch.unwrap()) {
            let ident = self.read_identifier();
            tok = Some(Token {
                token_type: lookup_ident(&ident),
                literal: ident,
            });
        } else if self.ch.unwrap().is_digit(10) {
            tok = Some(Token {
                token_type: TokenType::INT,
                literal: self.read_number(),
            });
        } else {
            tok = Some(Token {
                token_type: TokenType::ILLEGAL,
                literal: self.ch.unwrap().to_string(),
            });
            self.read_char();
        };

        tok
    }
}
