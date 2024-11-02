use crate::object;
use crate::{ast, eval::EvalError};
use crate::ast::Node;
use std::{any::Any, collections::HashMap};

#[derive(PartialEq)]
pub enum ObjType {
	ReturnValue,
	Integer,
	Boolean,
	Null,
	ObjVec,
	Error,
	Function,
}
#[derive(Clone)]
pub struct Env {
	pub store: HashMap<String, Box<dyn Obj>>,
	pub outer: Option<Box<Env>>,
}

impl Env {
	pub fn new(outer: Option<Box<Env>>) -> Self {
		let store = HashMap::new();
		Env { store , outer}
	}
	pub fn get(&self, name: String) -> Result<&Box<dyn Obj>, EvalError> {
		match (self.store.get(&name), &self.outer) {
			(None, None) => Err(EvalError::Undefined(String::from(""))),
			(Some(obj), _) => Ok(obj),
			(None, Some(out)) => out.get(name),
		}
	}
	pub fn set(&mut self, name: String, val: Box<dyn Obj>) {
		self.store.insert(name, val);
	}
}
impl Clone for Box<dyn Obj> {
	fn clone(&self) -> Self {
		self.clone_into_dyn()
	}
}

pub trait Obj: Any {
	fn get_type(&self) -> ObjType;
	fn inspect_obj(&self) -> String;
	fn as_any(&self) -> &dyn Any;
	fn clone_into_dyn(&self) -> Box<dyn Obj>;
}

impl Obj for Result<Box<dyn Obj>, EvalError> {
	fn get_type(&self) -> ObjType {
		match self {
			Err(_) => ObjType::Error,
			Ok(o) => o.get_type(),
		}
	}
	fn inspect_obj(&self) -> String {
		match self {
			Err(e) => e.get_err_msg(),
			Ok(o) => o.inspect_obj(),
		}
	}
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		panic!();
	}
}

#[derive(Clone)]
pub struct Function {
	pub params: Vec<ast::Identifier>,
	pub body: ast::BlockStatement,
	pub env: object::Env,
}
impl Obj for Function {
	fn get_type(&self) -> ObjType {
		ObjType::Function
	}

	fn inspect_obj(&self) -> String {
		let params = self
			.params
			.iter()
			.map(|p| p.value.clone())
			.fold(String::from(""), |acc, s| acc + "," + s.as_str());
		String::from("fn(") + &params + ")\n" + &self.body.string() + "\n"
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct ReturnValue {
	pub val: Box<dyn Obj>,
}
impl Obj for ReturnValue {
	fn get_type(&self) -> ObjType {
		ObjType::ReturnValue
	}

	fn inspect_obj(&self) -> String {
		self.val.inspect_obj()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct Integer {
	pub val: i64,
}
impl Obj for Integer {
	fn get_type(&self) -> ObjType {
		ObjType::Integer
	}
	fn inspect_obj(&self) -> String {
		format!("{0}", self.val)
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct Boolean {
	pub val: bool,
}
impl Obj for Boolean {
	fn get_type(&self) -> ObjType {
		ObjType::Boolean
	}
	fn inspect_obj(&self) -> String {
		format!("{0}", self.val)
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct Null {}
impl Obj for Null {
	fn get_type(&self) -> ObjType {
		ObjType::Null
	}
	fn inspect_obj(&self) -> String {
		String::from("null")
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		Box::new(self.clone())
	}
}
