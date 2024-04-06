#[cfg(test)]
mod parser_test;

use crate::{
        ast,
        ast::*,
        lexer::Lexer,
        token::{Token, TokenType},
};
use core::iter::Peekable;
use std::{collections::HashMap, string::ParseError};

struct Parser {
        lexer: Peekable<Lexer>,

        prefix_parse_fn: HashMap<TokenType, Box<ParsePrefixFunction>>,
        infix_parse_fn: HashMap<TokenType, Box<ParseInfixFunction>>,
}

#[derive(Debug)]
enum ParserError {
        UnexpectedEOF { expected: TokenType },
        UnexpectedToken { expected: TokenType, token: Token },
}

type ResultStatement = Result<Statement, ParserError>;
type ResultExpression = Result<Statement, ParserError>;

type ParsePrefixFunction = dyn Fn(&mut Parser) -> Result<Expression, ParserError>;
type ParseInfixFunction = dyn Fn(&mut Parser) -> Result<Expression, ParserError>;

impl Parser {
        fn new(lexer: Lexer) -> Parser {
                let lexer = lexer.peekable();
                let prefix_parse_fn: HashMap<TokenType, Box<ParsePrefixFunction>> = HashMap::from([
                        (TokenType::Ident, Box::new(Parser::parse_identifier_expression) as Box<ParsePrefixFunction>),       
                ]);
                let infix_parse_fn = HashMap::new();
                Parser {
                        lexer,
                        prefix_parse_fn,
                        infix_parse_fn,
                }
        }

        fn parse_program(self) -> Result<Program, ParserError> {
                let statements = self.collect::<Result<Vec<Statement>, ParserError>>()?;
                Ok(Program { statements })
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
        
        fn parse_identifier_expression(&mut self) -> Result<Expression, ParserError> {

        }

        fn parse_expression_statment(&self) -> Result<Statement, ParserError> {}
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


