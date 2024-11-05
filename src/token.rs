#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum TokenType {
	Illegal,
	Ident,
	Int,
	String,

	// Operators
	Assign,
	Plus,
	Minus,
	Bang,
	Asterisk,
	Slash,

	Lt,
	Gt,
	Eq,
	Noteq,

	Comma,
	Semicolon,
	Lparen,
	Rparen,
	Lbrace,
	Rbrace,

	// Keywords
	Function,
	Let,
	True,
	False,
	If,
	Else,
	Return,
}

impl TokenType {
	pub fn get_name(&self) -> String {
		match self {
			TokenType::Illegal => "Illegal Token".to_string(),
			TokenType::Ident => "Identifier Token".to_string(),
			TokenType::Int => "Integer Token".to_string(),
			TokenType::String => "String Token".to_string(),
			TokenType::Assign => "Assign = Token".to_string(),
			TokenType::Plus => "+ Token".to_string(),
			TokenType::Minus => "- Token".to_string(),
			TokenType::Bang => "! Token".to_string(),
			TokenType::Asterisk => "* Token".to_string(),
			TokenType::Slash => "/ Token".to_string(),
			TokenType::Lt => "< Token".to_string(),
			TokenType::Gt => "> Token".to_string(),
			TokenType::Eq => "= Token".to_string(),
			TokenType::Noteq => "!= Token".to_string(),
			TokenType::Comma => ", Token".to_string(),
			TokenType::Semicolon => "; Token".to_string(),
			TokenType::Lparen => "( Token".to_string(),
			TokenType::Rparen => ") Token".to_string(),
			TokenType::Lbrace => "{ Token".to_string(),
			TokenType::Rbrace => "} Token".to_string(),
			TokenType::Function => "Function Token".to_string(),
			TokenType::Let => "Let Token".to_string(),
			TokenType::True => "True Token".to_string(),
			TokenType::False => "False Token".to_string(),
			TokenType::If => "If Token".to_string(),
			TokenType::Else => "Else Token".to_string(),
			TokenType::Return => "Return Token".to_string(),
		}
	}
}

#[macro_export]
macro_rules! token {
	($tt: expr, $l:expr) => {{
		Token {
			token_type: $tt,
			literal: $l,
		}
	}};
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
	pub token_type: TokenType,
	pub literal: String,
}
