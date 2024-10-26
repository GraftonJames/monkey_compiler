use crate::parser::{Parser, ParserError};
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

		let program = par
			.parse_program()
			.statements
			.into_iter()
			.map(|s| s.string());

		for s in program {
			print!("{}\n", s);
		}
	}
}
