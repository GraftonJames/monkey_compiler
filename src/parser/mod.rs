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
        UnhandledToken { token: Token },
}

type ResultStatement = Result<Statement, ParserError>;
type ResultExpression = Result<Statement, ParserError>;
type OptionStatement = Option<Result<Statement, ParserError>>;

impl Parser {
        fn new(lexer: Lexer) -> Parser {
                let lexer = lexer.peekable();
                Parser { lexer }
        }

        fn parse_program(&mut self) -> Program {
                Program {
                        statements: self.collect(),
                }
        }

        fn token_expression_map(&self) -> fn(&Self) -> Box<dyn Node> {
                match self.lexer.peek().token_type {
                        Some(TokenType::Ident) => Self::parse_identifier,
                }
        }

        fn parse_let_statement(&mut self) -> ResultStatement {
                let token = self.expect_next_token(TokenType::Let)?;
                let name = self.parse_identifier()?;
                self.expect_next_token(TokenType::Assign)?;
                let value = self.parse_expression()?;
                Ok(Statement(Box::new(LetStatement { token, name, value })))
        }

        fn parse_return_statement(&mut self) -> ResultStatement {
                let token = self.expect_next_token(TokenType::Return)?;
                let value = self.parse_expression()?;
                Ok(Statement(Box::new(ReturnStatement { token, value })))
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
                match self.lexer.next() {
                        Some(token) if token.token_type == expected => Ok(token),
                        Some(token) => Err(ParserError::UnexpectedToken { expected, token }),
                        None => Err(ParserError::UnexpectedEOF { expected }),
                }
        }

        fn parse_expression_statment(&self) -> Result<Statement, ParserError> {

        }
}

impl Iterator for Parser {
        type Item = Result<Statement, ParserError>;
        fn next(&mut self) -> Option<Self::Item> {
                let token = self.lexer.peek()?;
                match token.token_type {
                        TokenType::Let => Some(self.parse_let_statement()),
                        TokenType::Return => Some(self.parse_return_statement()),
                        _ => Some(self.parse_expression_statment()),
                }
        }
}
