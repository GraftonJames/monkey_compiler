use crate::ast::Node;
use crate::eval::builtins::*;
use crate::object;
use crate::{ast, eval::EvalError};
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::Hasher;
use std::rc::Rc;
use std::{any::Any, collections::HashMap};

pub trait Hashable {
	fn hash_key(&self) -> HashKey;
}

#[derive(PartialEq, Clone, Eq, Hash)]
pub enum ObjType {
	ReturnValue,
	Integer,
	Boolean,
	String,
	Array,
	Hash,
	Null,
	ObjVec,
	Error,
	Function,
	BuiltinFunction,
}

impl ObjType {
	pub fn string(&self) -> String {
		match self {
			ObjType::ReturnValue => String::from("Return Value"),
			ObjType::Integer => String::from("Integer"),
			ObjType::Boolean => String::from("Boolean"),
			ObjType::String => String::from("String"),
			ObjType::Null => String::from("Null"),
			ObjType::ObjVec => String::from("ObjVec"),
			ObjType::Error => String::from("Error"),
			ObjType::Function => String::from("Function"),
			ObjType::BuiltinFunction => String::from("Builtin Function"),
			ObjType::Array => String::from("Array"),
			ObjType::Hash => String::from("Hash"),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HashKey {
	t: ObjType,
	val: u64,
}

#[derive(Clone)]
pub struct HashPair(pub Box<dyn Obj>, pub Box<dyn Obj>);

#[derive(Clone)]
pub struct Hash {
	pub pairs: HashMap<HashKey, HashPair>,
}
impl Obj for Hash {
	fn get_type(&self) -> ObjType {
		ObjType::Hash
	}

	fn inspect_obj(&self) -> String {
		let out = self
			.pairs
			.values()
			.fold(String::new(), |acc, HashPair(k, v)| {
				acc + format!(", {} : {}", k.inspect_obj(), v.inspect_obj())
					.as_str()
			});
		format!("[{}]", out)
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct Array {
	pub mems: VecDeque<Box<dyn Obj>>,
}
impl Obj for Array {
	fn get_type(&self) -> ObjType {
		ObjType::Array
	}
	fn inspect_obj(&self) -> String {
		let out = self.mems.iter().fold(String::new(), |acc, m| {
			acc + ", " + m.inspect_obj().as_str()
		});
		String::from("[") + out.as_str() + "]"
	}
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct Builtin {
	pub func: Rc<Box<dyn (Fn(Vec<Box<dyn Obj>>) -> Result<Box<dyn Obj>, EvalError>)>>,
}
impl Obj for Builtin {
	fn get_type(&self) -> ObjType {
		ObjType::BuiltinFunction
	}
	fn inspect_obj(&self) -> String {
		String::from("Builtin Function")
	}
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct Env {
	pub builtins: Rc<HashMap<String, Box<dyn Obj>>>,
	pub store: HashMap<String, Box<dyn Obj>>,
	pub outer: Option<Box<Env>>,
}

impl Env {
	pub fn new(outer: Option<Box<Env>>) -> Self {
		let builtins = match outer {
			None => Rc::new(get_builtins()),
			Some(ref e) => (e.builtins).clone(),
		};
		let store = HashMap::new();
		Env {
			builtins,
			store,
			outer,
		}
	}
	pub fn get(&self, name: String) -> Result<&Box<dyn Obj>, EvalError> {
		match self.builtins.get(&name) {
			Some(bi) => Ok(bi),
			None => self.get_desc(name),
		}
	}
	fn get_desc(&self, name: String) -> Result<&Box<dyn Obj>, EvalError> {
		match (self.store.get(&name), &self.outer) {
			(None, None) => {
				Err(EvalError::Undefined(String::from("Identifier not defined")))
			}
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
pub struct StringObj {
	pub val: String,
}
impl Obj for StringObj {
	fn get_type(&self) -> ObjType {
		ObjType::String
	}

	fn inspect_obj(&self) -> String {
		self.val.clone()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn clone_into_dyn(&self) -> Box<dyn Obj> {
		Box::new(self.clone())
	}
}
impl Hashable for StringObj {
	fn hash_key(&self) -> HashKey {
		let mut h = DefaultHasher::new();
		self.val.chars().for_each(|c| h.write_u32(c.into()));
		HashKey {
			t: ObjType::String,
			val: h.finish(),
		}
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
impl Hashable for Integer {
	fn hash_key(&self) -> HashKey {
		HashKey {
			t: ObjType::Integer,
			val: self.val.clone().try_into().unwrap(),
		}
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
impl Hashable for Boolean {
	fn hash_key(&self) -> HashKey {
		HashKey {
			t: ObjType::Boolean,
			val: self.val.clone().try_into().unwrap(),
		}
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
