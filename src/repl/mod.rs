use crate::object::Env;
use crate::object::Obj;
use crate::parser::Parser;
use crate::{ast::Node, lexer::Lexer};
use core::option::Iter;
use std::array::IntoIter;
use std::collections::VecDeque;
use std::io::Stdin;
use std::io::{stdin, stdout, Write};
use std::iter::*;
use std::str::Chars;

const PROMPT: &str = ">>";

struct ReplReader {
	chars: VecDeque<u8>,
}

impl Iterator for ReplReader {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		print!("hello");
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
		print!("new readr");
		ReplReader {
			chars: VecDeque::new(),
		}
	}
	fn get_line(&mut self) -> VecDeque<u8> {
		print!("{}", PROMPT);
		let s = &mut String::new();
		match stdin().read_line(s) {
			Err(_) => panic!(),
			_ => (),
		}
		s.clone().into_bytes().into()

	}
}

pub fn start() {
	print!("starting");
	let env = &mut Env::new(None);
	let mut reader = ReplReader::new();
	let lex = Lexer::new(reader);
	let par = Parser::new(lex);

	print!("parsing");
	let program = Box::new(par.parse_program())
		.into_eval_node()
		.eval(env)
		.inspect_obj();

	print!("{}\n", program);
}
