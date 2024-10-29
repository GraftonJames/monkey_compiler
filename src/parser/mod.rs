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
		TokenType::Lparen => CALL,
		_ => LOWEST,
	}
}
pub struct Parser {
	lexer: Peekable<Lexer>,
	prefix_parse_fn: HashMap<TokenType, ParseFunction>,
	infix_parse_fn: HashMap<TokenType, ParseInfixFunction>,
}

#[derive(Debug)]
pub enum ParserError {
	UnexpectedEOF(String),
	UnexpectedToken(String),
	IntParseError(String),
	NoPrefixParseFn(String),
	NoInfixParseFn(String),
	UnhandledError,
}

type BoxNode = Box<dyn Node>;
type ResultNode = Result<BoxNode, ParserError>;
type ParseFunction = fn(&mut Parser) -> ResultNode;
type ParseInfixFunction = fn(&mut Parser, BoxNode) -> ResultNode;

impl Parser {
	pub fn new(lexer: Lexer) -> Parser {
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
		prefix_parse_fn.insert(TokenType::True, Parser::parse_boolean as ParseFunction);
		prefix_parse_fn.insert(TokenType::False, Parser::parse_boolean as ParseFunction);
		prefix_parse_fn.insert(
			TokenType::Lparen,
			Parser::parse_grouped_expression as ParseFunction,
		);
		prefix_parse_fn.insert(TokenType::If, Parser::parse_if_statement as ParseFunction);
		prefix_parse_fn.insert(
			TokenType::Function,
			Parser::parse_function_literal as ParseFunction,
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
		infix_parse_fn.insert(
			TokenType::Lparen,
			Parser::parse_call_expression as ParseInfixFunction,
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
	pub fn parse_program(self) -> Program {
		let statements = self.collect::<Vec<ResultNode>>();
		Program { statements }
	}

	fn parse_let_statement(&mut self) -> ResultNode {
		let token = self.expect_next_token(TokenType::Let)?;
		let name = self.parse_identifier()?;
		self.expect_next_token(TokenType::Assign)?;
		let value = Box::new(self.parse_expression(LOWEST));
		self.expect_next_token(TokenType::Semicolon)?;
		Ok(Box::new(LetStatement { token, name, value }))
	}

	fn parse_return_statement(&mut self) -> ResultNode {
		let token = self.expect_next_token(TokenType::Return)?;
		let value = self.parse_expression(LOWEST)?;
		self.expect_next_token(TokenType::Semicolon)?;
		Ok(Box::new(ReturnStatement { token, value }))
	}

	fn parse_identifier(&mut self) -> Result<ast::Identifier, ParserError> {
		let token = self.expect_next_token(TokenType::Ident)?;
		Ok(Identifier {
			value: token.literal.clone(),
			token,
		})
	}

	fn parse_boolean(&mut self) -> ResultNode {
		let value = self.peek_token_is_type(TokenType::True)?;
		let token = self.lexer.next().ok_or(ParserError::UnexpectedEOF(
			"expected Boolean token found EOF".to_string(),
		))?;

		Ok(Box::new(BooleanLiteral { token, value }))
	}

	fn parse_if_statement(&mut self) -> ResultNode {
		let token = self.lexer.next().ok_or(ParserError::UnexpectedEOF(
			"Expected If Statement found EOF".to_string(),
		))?;

		self.expect_next_token(TokenType::Lparen)?;

		let condition = self.parse_expression(LOWEST)?;

		self.expect_next_token(TokenType::Rparen)?;

		let consequence = self.parse_block_statement()?;
		let alternative = self.parse_else_block()?;

		let expression = IfExpression {
			token,
			condition,
			consequence,
			alternative,
		};
		let expression = Box::new(expression);
		Ok(expression)
	}

	fn parse_else_block(&mut self) -> Result<Option<BlockStatement>, ParserError> {
		if !self.peek_token_is_type(TokenType::Else)? {
			return Ok(None);
		}
		self.lexer.next().unwrap();
		Ok(Some(self.parse_block_statement()?))
	}

	fn parse_grouped_expression(&mut self) -> ResultNode {
		self.expect_next_token(TokenType::Lparen)?;
		let expression = self.parse_expression(LOWEST);

		self.expect_next_token(TokenType::Rparen)?;
		expression
	}

	fn parse_expression(&mut self, precedence: u8) -> ResultNode {
		let token = self.peek_token()?;

		let prefix = self.prefix_parse_fn.get(&token.token_type).ok_or_else(|| {
			self.lexer.next();
			return ParserError::NoPrefixParseFn(format!(
				"no prefix parse function for {}",
				token.token_type.get_name()
			));
		})?;
		let mut left_exp: BoxNode = prefix(self)?;

		left_exp = self.parse_expression_infix(left_exp, precedence)?;

		Ok(left_exp)
	}

	fn parse_expression_infix(&mut self, left_exp: BoxNode, precedence: u8) -> ResultNode {
		if self.peek_token_is_type(TokenType::Semicolon)?
			|| &precedence >= &self.peek_precedence()
		{
			return Ok(left_exp);
		}
		let peek_token = self.peek_token()?;

		let infix = self
			.infix_parse_fn
			.get(&peek_token.token_type)
			.ok_or_else(|| {
				self.lexer.next();
				return ParserError::NoInfixParseFn(format!(
					"no infix parse function for {}",
					peek_token.token_type.get_name()
				));
			})?;
		let left_exp = infix(self, left_exp)?;
		self.parse_expression_infix(left_exp, precedence)
	}
	fn expect_next_token(&mut self, expected: TokenType) -> Result<Token, ParserError> {
		match self.lexer.next() {
			Some(token) if token.token_type == expected => Ok(token),
			Some(token) => Err(ParserError::UnexpectedToken(format!(
				"Unexpected {}, expected {}",
				token.literal,
				expected.get_name()
			))),
			None => Err(ParserError::UnexpectedEOF(
				"Expected token none found".to_string(),
			)),
		}
	}

	fn peek_token(&mut self) -> Result<Token, ParserError> {
		self.lexer
			.peek()
			.ok_or(ParserError::UnexpectedEOF("Unexpected EOF".to_string()))
			.cloned()
	}

	fn peek_token_is_type(&mut self, expected: TokenType) -> Result<bool, ParserError> {
		let token = self.lexer.peek().ok_or(ParserError::UnexpectedEOF(format!(
			"expected {} found EOF",
			expected.get_name()
		)))?;
		Ok(token.token_type == expected)
	}
	fn parse_identifier_expression(&mut self) -> ResultNode {
		let token = self.expect_next_token(TokenType::Ident)?;
		let value = token.literal.clone();
		Ok(Box::new(Identifier { token, value }) as Box<dyn Node>)
	}

	fn parse_expression_statement(&mut self) -> ResultNode {
		let token = self
			.lexer
			.peek()
			.ok_or(ParserError::UnhandledError)?
			.to_owned();
		let expression = self.parse_expression(LOWEST)?;
		self.expect_next_token(TokenType::Semicolon)?;
		Ok(Box::new(ExpressionStatement { expression, token }) as Box<dyn Node>)
	}

	fn parse_integer_literal(&mut self) -> ResultNode {
		let token = self.expect_next_token(TokenType::Int)?;
		let value = token.literal.parse().map_err(|_| {
			ParserError::IntParseError(format!(
				"Could not parse {} as integer",
				token.literal
			))
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

	fn parse_function_literal(&mut self) -> ResultNode {
		let token = self.expect_next_token(TokenType::Function)?;
		self.expect_next_token(TokenType::Lparen)?;

		let params = self.parse_funcion_parameters(Vec::new())?;
		let body = self.parse_block_statement()?;

		Ok(Box::new(FunctionLiteral {
			token,
			params,
			body,
		}))
	}

	fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
		let token = self.expect_next_token(TokenType::Lbrace)?;
		let statements = self.collect_statements(Vec::new(), TokenType::Rbrace)?;
		Ok(BlockStatement { token, statements })
	}

	fn collect_statements(
		&mut self,
		mut acc: Vec<ResultNode>,
		end_token: TokenType,
	) -> Result<Vec<ResultNode>, ParserError> {
		let peek_tok = self
			.lexer
			.peek()
			.ok_or(ParserError::UnexpectedEOF("Unexpected EOF".to_string()))?;

		if peek_tok.token_type == end_token {
			self.expect_next_token(end_token)?;
			return Ok(acc);
		}

		let stmt = self
			.next()
			.ok_or(ParserError::UnexpectedEOF("Unexpected EOF".to_string()))?;
		acc.push(stmt);
		self.collect_statements(acc, end_token)
	}

	fn parse_funcion_parameters(
		&mut self,
		mut params: Vec<Identifier>,
	) -> Result<Vec<Identifier>, ParserError> {
		let peek_tok = self.peek_token()?;
		if peek_tok.token_type == TokenType::Rparen {
			self.lexer.next();
			return Ok(params);
		}

		let token = self.expect_next_token(TokenType::Ident)?;
		let value = token.literal.clone();

		let ident = Identifier { token, value };
		params.push(ident);

		let peek_tok = self.peek_token()?;
		if peek_tok.token_type == TokenType::Comma {
			self.lexer.next();
		}

		return self.parse_funcion_parameters(params);
	}
	fn parse_call_expression(&mut self, function: BoxNode) -> ResultNode {
		let token = self.expect_next_token(TokenType::Lparen)?;
		let args = self.parse_call_args(Vec::new())?;
		Ok(Box::new(CallExpression {
			token,
			function,
			args,
		}))
	}
	fn parse_call_args(&mut self, mut left: Vec<BoxNode>) -> Result<Vec<BoxNode>, ParserError> {
		let peek_tok = self.peek_token()?;

		if peek_tok.token_type == TokenType::Rparen {
			self.lexer.next();
			return Ok(left);
		}

		self.lexer.next();
		let exp = self.parse_expression(LOWEST)?;
		left.push(exp);

		let peek_tok = self.peek_token()?;
		if peek_tok.token_type == TokenType::Comma {
			self.lexer.next();
			return self.parse_call_args(left);
		} else if peek_tok.token_type == TokenType::Rparen {
			return Ok(left);
		} else {
			return Err(ParserError::UnhandledError);
		}
	}
}

impl Iterator for Parser {
	type Item = ResultNode;
	fn next(&mut self) -> Option<Self::Item> {
		let token = self.lexer.peek()?;
		match token.token_type {
			TokenType::Let => Some(self.parse_let_statement()),
			TokenType::Return => Some(self.parse_return_statement()),
			_ => Some(self.parse_expression_statement()),
		}
	}
}
