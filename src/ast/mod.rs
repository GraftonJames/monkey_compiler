use super::token::Token;

pub trait Node {
        fn token_literal(&self) -> String;
}

pub struct Statement {
        pub node: Box<dyn Node>,
}
impl Node for Statement {
        fn token_literal(&self) -> String {
                self.node.token_literal()
        }
}

pub struct Expression {
        pub node: Box<dyn Node>,
}
impl Node for Expression {
        fn token_literal(&self) -> String {
                self.node.token_literal()
        }
}

pub struct Program {
        pub statements: Vec<Statement>,
}
impl Node for Program {
        fn token_literal(&self) -> String {
                if self.statements.len() > 0 {
                        self.statements[0].node.token_literal()
                } else {
                        "".to_string()
                }
        }
}

pub struct LetStatement {
        pub token: Token,
        pub name: Identifier,
        pub value: Expression,
}

impl Node for LetStatement {
        fn token_literal(&self) -> String {
                let mut lit = self.token.literal.clone();
                lit.push_str(" ");
                lit.push_str(&self.name.value);
                lit.push_str(" ");
                lit.push_str(&self.value.node.token_literal());
                lit
        }
}

pub struct ReturnStatement {
        pub token: Token,
        pub value: Expression,
}
impl Node for ReturnStatement {
        fn token_literal(&self) -> String {
                let mut lit = self.token.literal.clone();
                lit.push_str(" ");
                lit.push_str(&self.value.node.token_literal());
                lit
        }
}
pub struct Identifier {
        pub token: Token,
        pub value: String,
}
