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
	hm
}

fn len_fn(input: Vec<Box<dyn Obj>>) -> Result<Box<dyn Obj>, EvalError> {
	println!("called len");
	let input = match input.as_slice() {
		[i] => Ok(i),
		_ => Err(EvalError::IncorrectArgs(String::from("Len requires one arg"))),
	}?;
	let input = input
		.as_any()
		.downcast_ref::<StringObj>()
		.ok_or(EvalError::UnexpectedNode(String::from(
			"len must operate on a string",
		)))?;

	let val: i64 = input.val.len().try_into().unwrap();

	return Ok(Box::new(Integer { val }));
}
