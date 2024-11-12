use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::*;
use crate::object::*;

pub fn get_builtins() -> HashMap<String, Box<dyn Obj>> {
	let mut hm = HashMap::new();
	hm.insert(
		String::from("len"),
		Box::new(Builtin {
			func: Rc::new(Box::new(len_fn)),
		}) as Box<dyn Obj>,
	);
	hm.insert(
		String::from("first"),
		Box::new(Builtin {
			func: Rc::new(Box::new(first_fn)),
		}) as Box<dyn Obj>,
	);
	hm.insert(
		String::from("last"),
		Box::new(Builtin {
			func: Rc::new(Box::new(last_fn)),
		}) as Box<dyn Obj>,
	);
	hm.insert(
		String::from("rest"),
		Box::new(Builtin {
			func: Rc::new(Box::new(rest_fn)),
		}) as Box<dyn Obj>,
	);
	hm.insert(
		String::from("push"),
		Box::new(Builtin {
			func: Rc::new(Box::new(push_fn)),
		}) as Box<dyn Obj>,
	);
	hm
}

fn len_fn(input: Vec<Box<dyn Obj>>) -> Result<Box<dyn Obj>, EvalError> {
	println!("called len");
	let input = match input.as_slice() {
		[i] => Ok(i.clone()),
		_ => Err(EvalError::IncorrectArgs(String::from(
			"Len requires one arg",
		))),
	}?;
	match input.get_type() {
		ObjType::String => len_str(input),
		ObjType::Array => len_arr(input),
		_ => Err(EvalError::UnexpectedNode(String::from(
			"Argument is not of the correct type",
		))),
	}
}

fn len_str(input: Box<dyn Obj>) -> Result<Box<dyn Obj>, EvalError> {
	let input = input.as_any().downcast_ref::<StringObj>().unwrap();

	let val: i64 = input.val.len().try_into().unwrap();

	Ok(Box::new(Integer { val }))
}

fn len_arr(input: Box<dyn Obj>) -> Result<Box<dyn Obj>, EvalError> {
	let input = input.as_any().downcast_ref::<Array>().unwrap();

	let val: i64 = input.mems.len().try_into().unwrap();

	Ok(Box::new(Integer { val }))
}

fn first_fn(input: Vec<Box<dyn Obj>>) -> Result<Box<dyn Obj>, EvalError> {
	let input = match input.as_slice() {
		[i] => Ok(i),
		_ => Err(EvalError::IncorrectArgs(String::from(
			"Len requires one arg",
		))),
	}?;

	match input.get_type() {
		ObjType::Array => Ok(()),
		_ => Err(EvalError::IncorrectArgs(String::from(
			"first requires array argument",
		))),
	}?;

	input.as_any()
		.downcast_ref::<Array>()
		.unwrap()
		.mems
		.front()
		.ok_or(EvalError::IncorrectArgs(String::from(
			"array requires atleast one argument",
		)))
		.cloned()
}
fn last_fn(input: Vec<Box<dyn Obj>>) -> Result<Box<dyn Obj>, EvalError> {
	let input = match input.as_slice() {
		[i] => Ok(i),
		_ => Err(EvalError::IncorrectArgs(String::from(
			"Len requires one arg",
		))),
	}?;

	match input.get_type() {
		ObjType::Array => Ok(()),
		_ => Err(EvalError::IncorrectArgs(String::from(
			"first requires array argument",
		))),
	}?;

	input.as_any()
		.downcast_ref::<Array>()
		.unwrap()
		.mems
		.back()
		.ok_or(EvalError::IncorrectArgs(String::from(
			"array requires atleast one argument",
		)))
		.cloned()
}

fn rest_fn(input: Vec<Box<dyn Obj>>) -> Result<Box<dyn Obj>, EvalError> {
	let input = match input.as_slice() {
		[i] => Ok(i),
		_ => Err(EvalError::IncorrectArgs(String::from(
			"Len requires one arg",
		))),
	}?;
	match input.get_type() {
		ObjType::Array => Ok(()),
		_ => Err(EvalError::IncorrectArgs(String::from(
			"first requires array argument",
		))),
	}?;
	let mut input = input.as_any().downcast_ref::<Array>().unwrap().mems.clone();
	input.pop_front()
		.ok_or(EvalError::IncorrectArgs(String::from(
			"array requires atleast one argument",
		)))?;
	Ok(Box::new(Array {
		mems: input.clone(),
	}))
}

fn push_fn(input: Vec<Box<dyn Obj>>) -> Result<Box<dyn Obj>, EvalError> {
	let (&ref input, &ref object) = match input.as_slice() {
		[i, o] => Ok((i, o)),
		_ => Err(EvalError::IncorrectArgs(String::from(
			"Len requires one arg",
		))),
	}?;

	match input.get_type() {
		ObjType::Array => Ok(()),
		_ => Err(EvalError::IncorrectArgs(String::from(
			"first requires array argument",
		))),
	}?;

	let mut input = input.as_any().downcast_ref::<Array>().unwrap().mems.clone();

	input.push_back(object.clone());

	Ok(Box::new(Array {
		mems: input.clone(),
	}))
}
