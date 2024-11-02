use crate::ast::*;
use crate::{object::*, parser::ParserError};
use std::iter::zip;

type ResultObj = Result<Box<dyn Obj>, EvalError>;
#[derive(Debug)]
pub enum EvalError {
	ParserError(ParserError),
	UnexpectedNode(String),
	Undefined(String),
}

impl EvalError {
	pub fn get_err_type(&self) -> String {
		match self {
			EvalError::ParserError(_) => String::from("ParserError"),
			EvalError::UnexpectedNode(_) => String::from("UnexpectedNode"),
			EvalError::Undefined(_) => String::from("Undefined"),
		}
	}
	pub fn get_err_msg(&self) -> String {
		match self {
			EvalError::ParserError(e) => e.get_err_msg(),
			EvalError::UnexpectedNode(m) => m.to_string(),
			EvalError::Undefined(m) => m.to_string(),
		}
	}
}

pub trait EvalNode {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj;
}

pub struct Eval<N: Node> {
	pub node: N,
}

impl EvalNode for Eval<Program> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		self.node
			.statements
			.into_iter()
			.map(|s| -> ResultObj {
				match s {
					Err(e) => Err(EvalError::ParserError(e)),
					Ok(o) => o.into_eval_node().eval(env),
				}
			})
			.find(|n| match n.get_type() {
				ObjType::ReturnValue => true,
				ObjType::Error => true,
				_ => false,
			})
			.unwrap_or(Ok(Box::new(Null {})))
	}
}

impl EvalNode for Eval<ExpressionStatement> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		(*self).node.expression.into_eval_node().eval(env)
	}
}

impl EvalNode for Eval<IntegerLiteral> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		Ok(Box::new(Integer {
			val: self.node.value,
		}))
	}
}

impl EvalNode for Eval<CallExpression> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		let CallExpression {
			token: _,
			function,
			args,
		} = self.node;
		let function = function.into_eval_node().eval(env)?;
		let args: Result<Vec<_>,EvalError> = args
			.into_iter()
			.map(|a| a.into_eval_node().eval(env))
			.collect();
		let args = args?;
		apply_function(function, args)
	}
}

fn apply_function(function: Box<dyn Obj>, args: Vec<Box<dyn Obj>>) -> ResultObj {
	let function = function
		.as_any()
		.downcast_ref::<Function>()
		.ok_or(EvalError::UnexpectedNode(String::from(
			"Should be a function here"
		)))?;
	let Function { params, body, env } = function;

	let env = &mut Env::new(Some(Box::new(env.clone())));
	zip(params.into_iter(), args.into_iter()).for_each(|(p, a)| env.set(p.value.clone(), a));

	Box::new(Eval{ node: body.clone()}).eval(env)
}

impl EvalNode for Eval<IfExpression> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		let IfExpression {
			token: _,
			condition,
			consequence,
			alternative,
		} = (*self).node;

		let condition = condition
			.into_eval_node()
			.eval(env)?
			.as_any()
			.downcast_ref::<Boolean>()
			.ok_or(EvalError::UnexpectedNode(String::from(
				"Condition musst evaluate to a boolean value",
			)))?
			.val;

		if condition {
			return Box::new(consequence).into_eval_node().eval(env);
		}
		if let Some(n) = alternative {
			return Box::new(n).into_eval_node().eval(env);
		}
		Ok(Box::new(Null {}))
	}
}
impl EvalNode for Eval<FunctionLiteral> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		let FunctionLiteral {
			token: _,
			params,
			body,
		} = self.node;
		Ok(Box::new(Function { params, body, env: env.clone() }))
	}
}
impl EvalNode for Eval<BlockStatement> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		self.node
			.statements
			.into_iter()
			.scan(env, scan_node)
			.find(|n| n.as_ref().unwrap().get_type() == ObjType::ReturnValue)
			.unwrap_or(Ok(Box::new(Null {})))
	}
}

fn scan_node(
	env: &mut &mut Env,
	node: Result<Box<dyn Node + 'static>, ParserError>,
) -> Option<ResultObj> {
	let res = match node {
		Err(e) => Err(EvalError::ParserError(e)),
		Ok(o) => o.into_eval_node().eval(env),
	};
	Some(res)
}

impl EvalNode for Eval<BooleanLiteral> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		return Ok(Box::new(Boolean {
			val: self.node.value,
		}));
	}
}
impl EvalNode for Eval<LetStatement> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		let LetStatement {
			token: _,
			name,
			value,
		} = self.node;
		let value = value.into_eval_node().eval(env)?;
		env.set(name.value, value);
		Ok(Box::new(Null {}))
	}
}
impl EvalNode for Eval<ReturnStatement> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		Ok(Box::new(ReturnValue {
			val: self.node.value.into_eval_node().eval(env)?,
		}))
	}
}
impl EvalNode for Eval<Identifier> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		env.get(self.node.value).cloned()
	}
}
impl EvalNode for Eval<PrefixExpression> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		let PrefixExpression {
			operator,
			right,
			token: _,
		} = (*self).node;
		let right = right.into_eval_node().eval(env)?;
		match operator.as_str() {
			"!" => bang_op(right),
			"-" => minus_op(right),
			_ => Ok(Box::new(Null {})),
		}
	}
}
fn bang_op(right: Box<dyn Obj>) -> ResultObj {
	match right.inspect_obj().as_str() {
		"true" => Ok(Box::new(Boolean { val: false })),
		"false" => Ok(Box::new(Boolean { val: true })),
		"null" => Ok(Box::new(Boolean { val: true })),
		_ => Ok(Box::new(Boolean { val: false })),
	}
}
fn minus_op(right: Box<dyn Obj + 'static>) -> ResultObj {
	let val = -right
		.as_any()
		.downcast_ref::<Integer>()
		.ok_or(EvalError::UnexpectedNode(String::from(
			"- Must be followed by an integer",
		)))?
		.val;

	Ok(Box::new(Integer { val }))
}
impl EvalNode for Eval<InfixExpression> {
	fn eval(self: Box<Self>, env: &mut Env) -> ResultObj {
		let InfixExpression {
			operator,
			left,
			right,
			token: _,
		} = (*self).node;
		let left = left.into_eval_node().eval(env)?;
		let right = right.into_eval_node().eval(env)?;

		if left.get_type() == ObjType::Integer && right.get_type() == ObjType::Integer {
			return infix_eval_int(operator, left, right);
		}
		if left.get_type() == ObjType::Boolean && right.get_type() == ObjType::Boolean {
			return infix_eval_bool(operator, left, right);
		}

		Err(EvalError::UnexpectedNode(String::from(format!(
			"left ({0}) and right ({1}) operands are not correct",
			left.inspect_obj(),
			right.inspect_obj()
		))))
	}
}

fn infix_eval_bool(
	operator: String,
	left: Box<dyn Obj>,
	right: Box<dyn Obj>,
) -> Result<Box<dyn Obj>, EvalError> {
	let left = left
		.as_any()
		.downcast_ref::<Boolean>()
		.ok_or(EvalError::UnexpectedNode(format!(
			"{0} must operate on an integer to the left",
			operator,
		)))?
		.val;
	let right = right
		.as_any()
		.downcast_ref::<Boolean>()
		.ok_or(EvalError::UnexpectedNode(format!(
			"{0} must operate on an integer to the right",
			operator,
		)))?
		.val;

	match operator.as_str() {
		"==" => Ok(Box::new(Boolean { val: left == right })),
		"!=" => Ok(Box::new(Boolean { val: left != right })),
		_ => Err(EvalError::UnexpectedNode(String::from(
			"operator is not recognised as an infix expression",
		))),
	}
}

fn infix_eval_int(
	operator: String,
	left: Box<dyn Obj + 'static>,
	right: Box<dyn Obj + 'static>,
) -> Result<Box<dyn Obj>, EvalError> {
	let left = left
		.as_any()
		.downcast_ref::<Integer>()
		.ok_or(EvalError::UnexpectedNode(format!(
			"{0} must operate on an integer to the left",
			operator,
		)))?
		.val;
	let right = right
		.as_any()
		.downcast_ref::<Integer>()
		.ok_or(EvalError::UnexpectedNode(format!(
			"{0} must operate on an integer to the right",
			operator,
		)))?
		.val;

	match operator.as_str() {
		"+" => Ok(Box::new(Integer { val: left + right })),
		"-" => Ok(Box::new(Integer { val: left - right })),
		"*" => Ok(Box::new(Integer { val: left * right })),
		"/" => Ok(Box::new(Integer { val: left / right })),
		"<" => Ok(Box::new(Boolean { val: left < right })),
		">" => Ok(Box::new(Boolean { val: left > right })),
		"==" => Ok(Box::new(Boolean { val: left == right })),
		"!=" => Ok(Box::new(Boolean { val: left != right })),
		_ => Err(EvalError::UnexpectedNode(String::from(
			"operator is not recognised as an infix expression",
		))),
	}
}
