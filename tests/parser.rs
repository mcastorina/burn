use burn::lexer::Token;
use burn::parser::ast::{Expr, Lit};
use burn::parser::Parser;

#[test]
fn parse_arithmetic() {
    let mut parser = Parser::new("1 + 2");
    assert_eq!(parser.parse_expression().to_string(), "(1 + 2)");
}
