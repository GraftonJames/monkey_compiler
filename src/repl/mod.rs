use crate::object::Env;
use crate::object::Obj;
use crate::parser::Parser;
use crate::{ast::Node, lexer::Lexer};
use std::io::{stdin, stdout, Write};

const PROMPT: &str = ">>";

pub fn start() {
	loop {
		print!("{}", PROMPT);
		let buf: &mut String = &mut String::new();
		stdout().flush().expect("something went wrong");
		let _ = stdin().read_line(buf);

		let lex = Lexer::new(buf.to_string());
		let par = Parser::new(lex);

		let env = &mut Env::new(None);
		let program = Box::new(par.parse_program())
			.into_eval_node()
			.eval(env)
			.inspect();

		print!("{}\n", program);
	}
}
