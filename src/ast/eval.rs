use crate::ast::*;
use crate::{
	object::{self, Obj},
	parser::ParserError,
};

type BoxObj = Box<dyn Obj>;
type ResultObj = Result<Box<dyn Obj>, EvalError>;

enum EvalError {
	ParserError(ParserError),
}

trait EvalNode {
	fn eval(self) -> ResultObj; 
}

struct Eval<N: Node>(N);

fn wrap_eval_node<T: Node>(n: T) -> Eval<T> {
	EvalNode(n)
}

impl EvalNode for Eval<Program> {
	fn eval(self) -> ResultObj {
		self.0.statements
			.into_iter()
			.map(|s| -> ResultObj {
				match s {
					Err(e) => Err(EvalError::ParserError(e)),
					Ok(o) => Eval(*o).eval(),
				}
			})
			.collect()
	}
}

impl EvalNode for Eval<Box<ExpressionStatement>> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}

impl EvalNode for Eval<IntegerLiteral> {
	fn eval(self) -> ResultObj {
		object::Obj { val: self.0.value }
	}
}

impl EvalNode for Eval<CallExpression> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}

impl EvalNode for Eval<IfExpression> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<FunctionLiteral> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<BlockStatement> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<Boolean> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<LetStatement> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<ReturnStatement> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<Identifier> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<PrefixExpression> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}
impl EvalNode for Eval<InfixExpression> {
	fn eval(self) -> ResultObj {
		todo!()
	}
}
