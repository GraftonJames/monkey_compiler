use super::token::Token;
use crate::{
	eval::{Eval, EvalNode},
	parser::ParserError,
};
use std::any::Any;

pub trait Node: Any {
	fn token_literal(&self) -> String;
	fn string(&self) -> String;
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode>;
}

pub struct Program {
	pub statements: Vec<ResultNode>,
}

type BoxNode = Box<dyn Node>;
type ResultNode = Result<Box<dyn Node>, ParserError>;

pub trait BoxEvalWrapper {
	fn box_into_eval_node(self: Box<Self>) -> Box<dyn EvalNode>;
}
impl<N: Node + ?Sized> BoxEvalWrapper for Box<N> {
	fn box_into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		(*self).into_eval_node()
	}
}

impl Node for Program {
	fn token_literal(&self) -> String {
		if self.statements.len() > 0 {
			self.statements[0].token_literal()
		} else {
			"".to_string()
		}
	}
	fn string(&self) -> String {
		self.statements
			.iter()
			.clone()
			.fold("".to_string(), |acc, x| acc + "\n" + &x.string())
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

impl Node for ResultNode {
	fn token_literal(&self) -> String {
		match self {
			Ok(box_node) => box_node.token_literal(),
			Err(_) => "err".to_string(),
		}
	}
	fn string(&self) -> String {
		match self {
			Ok(ok_node) => return ok_node.string(),
			Err(ParserError::UnhandledError) => {
				panic!("Unexspected Result Encountered")
			}
			Err(ParserError::UnexpectedEOF(msg)) => return msg.clone().to_string(),
			Err(ParserError::IntParseError(msg)) => return msg.clone().to_string(),
			Err(ParserError::UnexpectedToken(msg)) => return msg.clone().to_string(),
			Err(ParserError::NoPrefixParseFn(msg)) => return msg.clone().to_string(),
			Err(ParserError::NoInfixParseFn(msg)) => return msg.clone().to_string(),
		}
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		panic!()
	}
}

pub struct CallExpression {
	pub token: Token,
	pub function: BoxNode,
	pub args: Vec<BoxNode>,
}

impl Node for CallExpression {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		let args = self
			.args
			.iter()
			.fold("".to_string(), |acc, a| acc + a.string().as_str() + " ,");
		let mut out = self.function.string();
		out.push_str("(");
		out.push_str(args.as_str());
		out.push_str(")");

		out
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct IfExpression {
	pub token: Token,
	pub condition: BoxNode,
	pub consequence: BlockStatement,
	pub alternative: Option<BlockStatement>,
}
impl Node for IfExpression {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		let out = "if ".to_string()
			+ &self.condition.string()
			+ " " + &self.consequence.string();

		match &self.alternative {
			Some(bs) => return out + bs.string().as_str(),

			None => return out,
		}
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct FunctionLiteral {
	pub token: Token,
	pub params: Vec<Identifier>,
	pub body: BlockStatement,
}
impl Node for FunctionLiteral {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		let params = self.params.iter().fold("".to_string(), |mut acc, p| {
			acc.push_str(p.string().as_str());
			acc.push_str(",");
			acc
		});

		let mut out = self.token_literal();
		out.push_str("(");
		out.push_str(params.as_str());
		out.push_str(")");
		out.push_str(self.body.string().as_str());

		out
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct BlockStatement {
	pub token: Token,
	pub statements: Vec<ResultNode>,
}
impl Node for BlockStatement {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		self.statements
			.iter()
			.clone()
			.fold("".to_string(), |acc, x| acc + "/n" + &x.string())
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct BooleanLiteral {
	pub token: Token,
	pub value: bool,
}
impl Node for BooleanLiteral {
	fn token_literal(&self) -> String {
		self.value.to_string()
	}

	fn string(&self) -> String {
		self.value.to_string()
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}
pub struct ExpressionStatement {
	pub token: Token,
	pub expression: BoxNode,
}
impl Node for ExpressionStatement {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		self.expression.string()
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct LetStatement {
	pub token: Token,
	pub name: Identifier,
	pub value: BoxNode,
}

impl Node for LetStatement {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		self.token_literal()
			+ " " + &self.name.token.literal
			+ " = " + &self.value.string()
			+ ";"
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct ReturnStatement {
	pub token: Token,
	pub value: BoxNode,
}
impl Node for ReturnStatement {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		self.token_literal() + " " + &self.value.string() + ";"
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct Identifier {
	pub token: Token,
	pub value: String,
}

impl Node for Identifier {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		self.value.clone()
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct IntegerLiteral {
	pub token: Token,
	pub value: i64,
}

impl Node for IntegerLiteral {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		self.token.literal.clone()
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct PrefixExpression {
	pub token: Token,
	pub operator: String,
	pub right: BoxNode,
}

impl Node for PrefixExpression {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		"(".to_owned() + &self.operator + &self.right.string() + ")"
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}

pub struct InfixExpression {
	pub token: Token,
	pub left: BoxNode,
	pub operator: String,
	pub right: BoxNode,
}

impl Node for InfixExpression {
	fn token_literal(&self) -> String {
		self.token.literal.clone()
	}

	fn string(&self) -> String {
		"(".to_owned() + &self.left.string() + &self.operator + &self.right.string() + ")"
	}
	fn into_eval_node(self: Box<Self>) -> Box<dyn EvalNode> {
		Box::new(Eval { node: *self })
	}
}
