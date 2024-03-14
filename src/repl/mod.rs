use crate::lexer::Lexer;
use std::io::{stdin, stdout, Write};

const PROMPT: &str = ">>";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        let buf: &mut String = &mut String::new();
        stdout().flush().expect("something went wrong");
        let _ = stdin().read_line(buf);
        for tok in Lexer::new(buf.to_string()) {
            print!("{:?}\n", tok);
        }
    }
}
