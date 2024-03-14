use crate::{ast::{Program, Statement}, lexer::Lexer, token::Token};

struct Parser {
    lexer: Lexer,
    token: Option<Token>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let token = lexer.next();
        Parser { lexer, token }
    }
    fn parse_program(&self) -> Option<Program> {
        let program = Program { statements: vec!()};
        
        while let self.token = Some(token) {
            statement = self.parse_statement();

        }
    }
    
    fn parse_statement(&self) -> Box<Statement> {
        
    }
}
