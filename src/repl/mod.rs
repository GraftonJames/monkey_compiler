use crate::lexer::Lexer;
use std::io::BufRead;

const PROMPT: &str = ">>";

pub fn start(input: impl BufRead) {
    print!("{}", PROMPT);
    for tok in input.lines().flat_map(|l| Lexer::new(l.unwrap())) {
        print!("{:?}", tok);
    }
}
