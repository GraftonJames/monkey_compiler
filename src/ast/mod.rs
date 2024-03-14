use super::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

pub struct Statement {
    pub node: dyn Node,
}

trait Expression {
}

pub struct Program {
    pub statements: Vec<Box<Statement>>,
}
impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].node.token_literal()
        } else {
            String::from("")
        }
    }
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: dyn Expression,
}

impl LetStatement {
    fn statement_node() {}
}

struct Identifier {
    token: Token,
    value: String,
}
