use std::any::Any;

use crate::eval::EvalError;

#[derive(PartialEq)]
pub enum ObjType {
	Integer,
	Boolean,
	Null,
	ObjVec,
}

pub trait Obj: Any {
	fn get_type(&self) -> ObjType;
	fn inspect(&self) -> String;
	fn as_any(&self) -> &dyn Any;
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

	fn as_any(&self) ->  &dyn Any{
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

	fn as_any(&self) ->  &dyn Any{
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

	fn as_any(&self) ->  &dyn Any{
		self
	}
}
