use super::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub struct Program {
    pub statements: Vec<BoxNode>,
}

type BoxNode = Box<dyn Node>;
impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }

    fn string(&self) -> String {
        self.statements
            .iter()
            .clone()
            .fold("".to_string(), |acc, x| acc + "/n" + &x.string())
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: BoxNode,
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.expression.string()
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: BoxNode,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token_literal() + " " + &self.name.token.literal + " = " + &self.value.string() + ";"
    }
}

pub struct ReturnStatement {
    pub token: Token,
    pub value: BoxNode,
}
impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token_literal() + " " + &self.value.string() + ";"
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}
