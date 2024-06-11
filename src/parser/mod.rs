use crate::{
    ast,
    ast::*,
    lexer::Lexer,
    token::{Token, TokenType},
};
use core::iter::Peekable;
use std::collections::HashMap;

const LOWEST: u8 = 1;
const EQUALS: u8 = 2;
const LESSGREATER: u8 = 3;
const SUM: u8 = 4;
const PRODUCT: u8 = 5;
const PREFIX: u8 = 6;
const CALL: u8 = 7;

fn get_precedence(token_type: &TokenType) -> u8 {
    match token_type {
        TokenType::Plus | TokenType::Minus => SUM,
        TokenType::Asterisk | TokenType::Slash => PRODUCT,
        TokenType::Lt | TokenType::Gt => LESSGREATER,
        TokenType::Eq | TokenType::Noteq => EQUALS,
        _ => LOWEST,
    }
}
struct Parser {
    lexer: Peekable<Lexer>,

    prefix_parse_fn: HashMap<TokenType, ParseFunction>,
    infix_parse_fn: HashMap<TokenType, ParseInfixFunction>,
}

#[derive(Debug)]
enum ParserError {
    UnexpectedEOF(String),
    UnexpectedToken(String),
    IntParseError(String),
    NoPrefixParseFn(String),
    UnhandledError,
}

type BoxNode = Box<dyn Node>;
type ResultNode = Result<BoxNode, ParserError>;
type ParseFunction = fn(&mut Parser) -> ResultNode;
type ParseInfixFunction = fn(&mut Parser, BoxNode) -> ResultNode;
impl Parser {
    fn new(lexer: Lexer) -> Parser {
        let lexer = lexer.peekable();
        let mut prefix_parse_fn: HashMap<TokenType, ParseFunction> = HashMap::new();
        prefix_parse_fn.insert(
            TokenType::Ident,
            Parser::parse_identifier_expression as ParseFunction,
        );
        prefix_parse_fn.insert(
            TokenType::Int,
            Parser::parse_integer_literal as ParseFunction,
        );
        prefix_parse_fn.insert(
            TokenType::Bang,
            Parser::parse_prefix_expression as ParseFunction,
        );
        prefix_parse_fn.insert(
            TokenType::Minus,
            Parser::parse_prefix_expression as ParseFunction,
        );

        let mut infix_parse_fn: HashMap<TokenType, ParseInfixFunction> = HashMap::new();
        infix_parse_fn.insert(
            TokenType::Plus,
            Parser::parse_infix_expression as ParseInfixFunction,
        );
        infix_parse_fn.insert(
            TokenType::Minus,
            Parser::parse_infix_expression as ParseInfixFunction,
        );
        infix_parse_fn.insert(
            TokenType::Slash,
            Parser::parse_infix_expression as ParseInfixFunction,
        );
        infix_parse_fn.insert(
            TokenType::Asterisk,
            Parser::parse_infix_expression as ParseInfixFunction,
        );
        infix_parse_fn.insert(
            TokenType::Eq,
            Parser::parse_infix_expression as ParseInfixFunction,
        );
        infix_parse_fn.insert(
            TokenType::Noteq,
            Parser::parse_infix_expression as ParseInfixFunction,
        );
        infix_parse_fn.insert(
            TokenType::Lt,
            Parser::parse_infix_expression as ParseInfixFunction,
        );
        infix_parse_fn.insert(
            TokenType::Gt,
            Parser::parse_infix_expression as ParseInfixFunction,
        );

        Parser {
            lexer,
            prefix_parse_fn,
            infix_parse_fn,
        }
    }
    fn peek_precedence(&mut self) -> u8 {
        let token = self.lexer.peek().unwrap();
        get_precedence(&token.token_type)
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

    fn parse_expression(&mut self, precedence: u8) -> ResultNode {
        let token = self.lexer.next().ok_or(ParserError::UnhandledError)?;
        let prefix =
            self.prefix_parse_fn
                .get(&token.token_type)
                .ok_or(ParserError::NoPrefixParseFn(
                    "no prefix parse function for token".to_string(),
                ))?;
        let mut left_exp: BoxNode = prefix(self)?;

        while !&self.peek_token_is_type(TokenType::Semicolon)?
            && &precedence < &self.peek_precedence()
        {
            let peek_token = self.lexer.peek().ok_or(ParserError::UnhandledError)?;
            let infix = self.infix_parse_fn.get(&peek_token.token_type).ok_or(
                ParserError::NoPrefixParseFn("no infix parse function for token".to_string()),
            )?;

            left_exp = infix(self, left_exp)?;
        }
        Ok(left_exp)
    }

    fn expect_next_token(&mut self, expected: TokenType) -> Result<Token, ParserError> {
        match self.lexer.next() {
            Some(token) if token.token_type == expected => Ok(token),
            Some(_) => Err(ParserError::UnexpectedToken(
                "Unexpected token type".to_string(),
            )),
            None => Err(ParserError::UnexpectedEOF(
                "Expected token none found".to_string(),
            )),
        }
    }

    fn peek_token_is_type(&mut self, expected: TokenType) -> Result<bool, ParserError> {
        let token = self.lexer.peek().ok_or(ParserError::UnhandledError)?;
        Ok(token.token_type == expected)
    }
    fn parse_identifier_expression(&mut self) -> ResultNode {
        let token = self.expect_next_token(TokenType::Ident)?;
        let value = token.literal.clone();
        Ok(Box::new(Identifier { token, value }) as Box<dyn Node>)
    }

    fn parse_expression_statment(&mut self) -> ResultNode {
        let expression = self.parse_expression()?;
        let token = self
            .lexer
            .peek()
            .ok_or(ParserError::UnhandledError)?
            .to_owned();
        Ok(Box::new(ExpressionStatement { expression, token }) as Box<dyn Node>)
    }

    fn parse_integer_literal(&mut self) -> ResultNode {
        let token = self.expect_next_token(TokenType::Int)?;
        let value = token.literal.parse().map_err(|_| {
            ParserError::IntParseError(format!("Could not parse {} as integer", token.literal))
        })?;
        Ok(Box::new(IntegerLiteral { token, value }))
    }

    fn parse_prefix_expression(&mut self) -> ResultNode {
        let token = self.lexer.next().unwrap();
        let operator = token.literal.clone();
        let right = self.parse_expression(PREFIX)?;
        Ok(Box::new(PrefixExpression {
            token,
            operator,
            right,
        }))
    }

    fn parse_infix_expression(&mut self, left: BoxNode) -> ResultNode {
        let token = self.lexer.next().unwrap();
        let precedence = get_precedence(&token.token_type);
        let right = self.parse_expression(precedence)?;
        let operator = token.literal.clone();
        Ok(Box::new(InfixExpression {
            token,
            left,
            operator,
            right,
        }))
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
