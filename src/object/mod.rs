use std::{any::Any, collections::HashMap};

use crate::eval::EvalError;

#[derive(PartialEq)]
pub enum ObjType {
	ReturnValue,
	Integer,
	Boolean,
	Null,
	ObjVec,
	Error,
}

pub struct Env {
	store: HashMap<String, Box<dyn Obj>>,
}

impl Env {
	pub fn new() -> Self {
		let store = HashMap::new();
		Env {store}
	}
	pub fn get(&self, name: String) -> Result<&Box<dyn Obj>, EvalError> {
		match self.store.get(&name) {
			None => Err(EvalError::Undefined(String::from(""))),
			Some(o) => Ok(o),
		}
	}
	pub fn set(&mut self, name: String, val: Box<dyn Obj>) {
		self.store.insert(name, val);
	}
}

pub trait Obj: Any {
	fn get_type(&self) -> ObjType;
	fn inspect(&self) -> String;
	fn as_any(&self) -> &dyn Any;
}

impl Obj for Result<Box<dyn Obj>, EvalError> {
	fn get_type(&self) -> ObjType {
		match self {
			Err(_) => ObjType::Error,
			Ok(o) => o.get_type(),
		}
	}

	fn inspect(&self) -> String {
		match self {
			Err(e) => e.get_err_msg(),
			Ok(o) => o.inspect(),
		}
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

pub struct ObjVec {
	pub val: Vec<Result<Box<dyn Obj>, EvalError>>,
}
impl Obj for ObjVec {
	fn get_type(&self) -> ObjType {
		ObjType::ObjVec
	}

	fn inspect(&self) -> String {
		format!(
			"{0}",
			self.val.iter().clone().fold("".to_string(), |acc, o| acc
				+ "\n" + o
				.as_ref()
				.unwrap()
				.inspect()
				.as_str())
		)
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}
pub struct ReturnValue {
	pub val: Box<dyn Obj>,
}
impl Obj for ReturnValue {
	fn get_type(&self) -> ObjType {
		ObjType::ReturnValue
	}

	fn inspect(&self) -> String {
		self.val.inspect()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}
pub struct Integer {
	pub val: i64,
}

impl Obj for Integer {
	fn get_type(&self) -> ObjType {
		ObjType::Integer
	}
	fn inspect(&self) -> String {
		format!("{0}", self.val)
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

pub struct Boolean {
	pub val: bool,
}

impl Obj for Boolean {
	fn get_type(&self) -> ObjType {
		ObjType::Boolean
	}
	fn inspect(&self) -> String {
		format!("{0}", self.val)
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

pub struct Null {}

impl Obj for Null {
	fn get_type(&self) -> ObjType {
		ObjType::Null
	}
	fn inspect(&self) -> String {
		String::from("null")
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}
