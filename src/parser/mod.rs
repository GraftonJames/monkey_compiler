use crate::{
    ast,
    ast::*,
    lexer::Lexer,
    token::{Token, TokenType},
};
use core::iter::Peekable;
use std::{collections::HashMap, ops::Deref};

struct Parser {
    lexer: Peekable<Lexer>,

    prefix_parse_fn: HashMap<TokenType, ParseFunction>,
    infix_parse_fn: HashMap<TokenType, ParseFunction>,
}

#[derive(Debug)]
enum ParserError {
    UnexpectedEOF { expected: TokenType },
    UnexpectedToken { expected: TokenType, token: Token },
    UnhandledError,
}

type BoxNode = Box<dyn Node>;
type ResultNode = Result<BoxNode, ParserError>;
type ParseFunction = fn(&mut Parser) -> Result<BoxNode, ParserError>;

impl Parser {
    fn new(lexer: Lexer) -> Parser {
        let lexer = lexer.peekable();
        let prefix_parse_fn: HashMap<TokenType, ParseFunction> = HashMap::from([(
            TokenType::Ident,
            Parser::parse_identifier_expression as ParseFunction,
        )]);
        let infix_parse_fn = HashMap::new();
        Parser {
            lexer,
            prefix_parse_fn,
            infix_parse_fn,
        }
    }

    fn parse_program(self) -> Result<Program, ParserError> {
        let statements = self.collect::<Result<Vec<BoxNode>, ParserError>>()?;
        Ok(Program { statements })
    }

    fn parse_let_statement(&mut self) -> ResultNode {
        let token = self.expect_next_token(TokenType::Let)?;
        let name = self.parse_identifier()?;
        self.expect_next_token(TokenType::Assign)?;
        let value = self.parse_expression()?;
        Ok(Box::new(LetStatement { token, name, value }))
    }

    fn parse_return_statement(&mut self) -> ResultNode {
        let token = self.expect_next_token(TokenType::Return)?;
        let value = self.parse_expression()?;
        Ok(Box::new(ReturnStatement { token, value }))
    }

    fn parse_identifier(&mut self) -> Result<ast::Identifier, ParserError> {
        let token = self.expect_next_token(TokenType::Ident)?;
        Ok(Identifier {
            value: token.literal.clone(),
            token,
        })
    }

    fn parse_expression(&mut self) -> ResultNode {
        let token = self.lexer.peek().ok_or(ParserError::UnhandledError)?;
        let prefix = self
            .prefix_parse_fn
            .get(&token.token_type)
            .ok_or(ParserError::UnhandledError)?;
        let left_exp = prefix(self);
        left_exp
    }

    fn expect_next_token(&mut self, expected: TokenType) -> Result<Token, ParserError> {
        match self.lexer.next() {
            Some(token) if token.token_type == expected => Ok(token),
            Some(token) => Err(ParserError::UnexpectedToken { expected, token }),
            None => Err(ParserError::UnexpectedEOF { expected }),
        }
    }

    fn parse_identifier_expression(&mut self) -> ResultNode {
        let token = self.lexer.next().unwrap();
        let value = token.literal.clone();
        Ok(Box::new(Identifier { token, value }) as Box<dyn Node>)
    }

    fn parse_expression_statment(&mut self) -> ResultNode {
        let expression = self.parse_expression()?;
        let token = self.lexer.peek().ok_or(ParserError::UnhandledError)?;
        let token = token;
        Ok(Box::new(ExpressionStatement { expression, token }) as Box<dyn Node>)
    }
}

impl Iterator for Parser {
    type Item = ResultNode;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.peek()?;
        match token.token_type {
            TokenType::Let => Some(self.parse_let_statement()),
            TokenType::Return => Some(self.parse_return_statement()),
            _ => Some(self.parse_expression_statment()),
        }
    }
}
