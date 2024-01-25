use std::io::Stdin;

mod lexer;
mod repl;
mod token;

fn main() {
    let input: Stdin = std::io::stdin();
    repl::start(input.lock());
}
