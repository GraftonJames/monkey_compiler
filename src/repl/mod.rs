use crate::object::Env;
use crate::object::Obj;
use crate::parser::Parser;
use crate::{ast::Node, lexer::Lexer};
use std::collections::VecDeque;
use std::io;
use std::io::stdin;
use std::io::Write;
use std::iter::*;

use crate::object::ObjType;

const PROMPT: &str = ">>";

struct ReplReader {
	chars: VecDeque<u8>,
}

impl Iterator for ReplReader {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		match self.chars.pop_front() {
			None => {
				self.queue();
				self.next()
			}
			Some(c) => Some(char::from(c)),
		}
	}
}

impl ReplReader {
	fn queue(&mut self) {
		self.chars = self.get_line();
	}
	fn new() -> Self {
		ReplReader {
			chars: VecDeque::new(),
		}
	}
	fn get_line(&mut self) -> VecDeque<u8> {
		print!("{}", PROMPT);
		io::stdout().flush().ok().expect("Could not flush");
		let s = &mut String::new();
		match stdin().read_line(s) {
			Err(_) => panic!(),
			_ => (),
		}
		s.clone().into_bytes().into()
	}
}

pub fn start() {
	let env = &mut Env::new(None);
	let reader = ReplReader::new();
	let lex = Lexer::new(reader);
	let par = Parser::new(lex);

	let mut eval = par.map(|n| n.unwrap().into_eval_node().eval(env));
	loop {
		let msg = match eval.find(|e| e.get_type() == ObjType::ReturnValue).unwrap() {
			Ok(r) => r.inspect_obj(),
			Err(e) => e.get_err_msg(),
		};
		print!("{}<<<\n", msg);
	}
}
