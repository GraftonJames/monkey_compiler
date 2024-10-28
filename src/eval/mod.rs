use crate::ast::*;
use crate::{object::*, parser::ParserError};
use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;

type BoxObj = Box<dyn Obj>;
type ResultObj = Result<Box<dyn Obj>, EvalError>;
#[derive(Debug)]
pub enum EvalError {
	ParserError(ParserError),
	UnexpectedNode(String),
}

pub trait EvalNode {
	fn eval(self: Box<Self>) -> ResultObj;
}

pub struct Eval<N: Node> {
	pub node: N,
}

impl EvalNode for Eval<Program> {
	fn eval(self: Box<Self>) -> ResultObj {
		let val = self
			.node
			.statements
			.into_iter()
			.map(|s| -> ResultObj {
				match s {
					Err(e) => Err(EvalError::ParserError(e)),
					Ok(o) => o.into_eval_node().eval(),
				}
			})
			.collect();
		Ok(Box::new(ObjVec { val }))
	}
}

impl EvalNode for Eval<ExpressionStatement> {
	fn eval(self: Box<Self>) -> ResultObj {
		(*self).node.expression.into_eval_node().eval()
	}
}

impl EvalNode for Eval<IntegerLiteral> {
	fn eval(self: Box<Self>) -> ResultObj {
		Ok(Box::new(Integer {
			val: self.node.value,
		}))
	}
}

impl EvalNode for Eval<CallExpression> {
	fn eval(self: Box<Self>) -> ResultObj {
		todo!()
	}
}

impl EvalNode for Eval<IfExpression> {
	fn eval(self: Box<Self>) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<FunctionLiteral> {
	fn eval(self: Box<Self>) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<BlockStatement> {
	fn eval(self: Box<Self>) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<BooleanLiteral> {
	fn eval(self: Box<Self>) -> ResultObj {
		return Ok(Box::new(Boolean { val: true }));
	}
}
impl EvalNode for Eval<LetStatement> {
	fn eval(self: Box<Self>) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<ReturnStatement> {
	fn eval(self: Box<Self>) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<Identifier> {
	fn eval(self: Box<Self>) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<PrefixExpression> {
	fn eval(self: Box<Self>) -> ResultObj {
		let PrefixExpression {
			operator,
			right,
			token: _,
		} = (*self).node;
		let right = right.into_eval_node().eval()?;
		match operator.as_str() {
			"!" => bang_op(right),
			"-" => minus_op(right),
			_ => Ok(Box::new(Null {})),
		}
	}
}
fn bang_op(right: Box<dyn Obj>) -> ResultObj {
	match right.inspect().as_str() {
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
	fn eval(self: Box<Self>) -> ResultObj {
		todo!()
	}
}
