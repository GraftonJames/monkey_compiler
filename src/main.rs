mod lexer;
mod token;

fn main() {
    let lexer = lexer::Lexer::new(
        "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        
        10 != 9;
        10 == 10;
    ",
    );

    for c in lexer {
        print!("{}", c.literal);
    }
}
