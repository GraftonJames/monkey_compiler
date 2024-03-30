#[cfg(test)]
mod parser_test;

use crate::{
        ast,
        ast::*,
        lexer::Lexer,
        token::{Token, TokenType},
};
use core::iter::Peekable;

struct Parser {
        lexer: Peekable<Lexer>,
}

#[derive(Debug)]
enum ParserError {
        UnexpectedEOF { expected: TokenType },
        UnexpectedToken { expected: TokenType, token: Token },
}

type ResultStatement = Result<Option<Statement>, ParserError>;

impl Parser {
        fn new(lexer: Lexer) -> Parser {
                let lexer = lexer.peekable();
                Parser { lexer }
        }

        fn parse_program(&mut self) -> Program {
                Program { statements: self.collect() }
        }

        fn parse_let_statement(&mut self) -> ResultStatement {
                let token = self.expect_next_token(TokenType::Let)?;
                let name = self.parse_identifier()?;
                self.expect_next_token(TokenType::Assign)?;
                let value = self.parse_expression()?;
                Ok(Some(Statement {
                        node: Box::new(LetStatement { token, name, value }),
                }))
        }

        fn parse_return_statement(&mut self) -> ResultStatement {
                let token = self.expect_next_token(TokenType::Return)?;
                let value = self.parse_expression()?;
                Ok(Some(Statement {
                        node: Box::new(ReturnStatement { token, value }),
                }))
        }

        fn parse_identifier(&mut self) -> Result<ast::Identifier, ParserError> {
                let token = self.expect_next_token(TokenType::Ident)?;
                Ok(Identifier {
                        value: token.literal.clone(),
                        token,
                })
        }

        fn parse_expression(&self) -> Result<ast::Expression, ParserError> {
                todo!()
        }

        fn expect_next_token(&mut self, expected: TokenType) -> Result<Token, ParserError> {
                let token = self.lexer.next();
                match token {
                        Some(token) if token.token_type == expected => Ok(token),
                        None => Err(ParserError::UnexpectedEOF { expected }),
                        Some(token) => Err(ParserError::UnexpectedToken { expected, token }),
                }
        }
}

impl Iterator for Parser {
        type Item = Statement;
        fn next(&mut self) -> Option<Self::Item> {
                let token = self.lexer.peek()?;
                match token.token_type {
                        TokenType::Let => self.parse_let_statement().unwrap(),
                        TokenType::Return => self.parse_return_statement().unwrap(),
                        _ => None,
                }
        }
}
