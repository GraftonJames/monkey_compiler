use crate::lexer::*;
use crate::parser::*;

#[test]
fn test_let_statements() {
        let input = "
                let x = 5;
                let y = 10;
                let foobar = 5165157;
        ";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        let program = program.statements;

        assert_eq!(program.len(), 3);

        let expected = vec![("x", "5"), ("y", "10"), ("foobar", "5165157")];
        for stmt in program {
                assert_eq!(stmt.node.token_literal(), "");
        }
}
