use std::{iter::Peekable, ops::Deref};

use crate::token;
use crate::token::*;

pub struct Lexer {
    input: Peekable<std::vec::IntoIter<char>>,
    ch: Option<char>,
}
fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
fn lookup_ident(ident: &String) -> TokenType {
    match ident.deref() {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        "true" => TokenType::True,
        "false" => TokenType::False,
        _ => TokenType::Ident,
    }
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let input: Vec<char> = input.chars().collect();
        let mut input = input.into_iter().peekable();
        let ch = input.next();
        Lexer { input, ch }
    }

    fn read_char(&mut self) {
        self.ch = self.input.next();
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input.peek().copied()
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
        while self.ch.is_some() && self.ch.unwrap().is_ascii_digit() {
            int.push(self.ch.unwrap());
            self.read_char();
        }
        int.into_iter().collect()
    }

    fn read_single_char_token(&mut self) -> Option<Token> {
        let ch = self.ch.unwrap();
        match ch {
            '=' => {
                let ch_next = self.peek_char();
                if ch_next.is_some() && ch_next.unwrap() == '=' {
                    self.read_char();
                    let ch_next = self.ch.unwrap();
                    return Some(token!(TokenType::Eq, format!("{}{}", ch, ch_next)));
                }
                Some(token!(TokenType::Assign, ch.to_string()))
            }
            '+' => Some(token!(TokenType::Plus, ch.to_string())),
            '-' => Some(token!(TokenType::Minus, ch.to_string())),
            '!' => {
                let ch_next = self.peek_char();
                if ch_next.is_some() && ch_next.unwrap() == '=' {
                    self.read_char();
                    let ch_next = self.ch.unwrap();
                    return Some(token!(TokenType::Noteq, format!("{}{}", ch, ch_next)));
                }
                Some(token!(TokenType::Bang, ch.to_string()))
            }
            '/' => Some(token!(TokenType::Slash, ch.to_string())),
            '*' => Some(token!(TokenType::Asterisk, ch.to_string())),
            '<' => Some(token!(TokenType::Lt, ch.to_string())),
            '>' => Some(token!(TokenType::Gt, ch.to_string())),
            ';' => Some(token!(TokenType::Semicolon, ch.to_string())),
            '(' => Some(token!(TokenType::Lparen, ch.to_string())),
            ')' => Some(token!(TokenType::Rparen, ch.to_string())),
            ',' => Some(token!(TokenType::Comma, ch.to_string())),
            '{' => Some(token!(TokenType::Lbrace, ch.to_string())),
            '}' => Some(token!(TokenType::Rbrace, ch.to_string())),
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

impl Iterator for Lexer {
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
        } else if self.ch.unwrap().is_ascii_digit() {
            tok = Some(Token {
                token_type: TokenType::Int,
                literal: self.read_number(),
            });
        } else {
            tok = Some(Token {
                token_type: TokenType::Illegal,
                literal: self.ch.unwrap().to_string(),
            });
            self.read_char();
        };

        tok
    }
}
