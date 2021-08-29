use burn::lexer::Token;
use burn::parser::ast::{Expr, Lit};
use burn::parser::Parser;

#[test]
fn parse_literals() {
    fn parse(input: &str) -> Expr {
        let mut parser = Parser::new(input);
        parser.parse_expression()
    }
    assert_eq!(parse("1  ").to_string(), "1");
    assert_eq!(parse("  \"string\"").to_string(), "\"string\"");
    assert_eq!(
        parse("'string with spaces'").to_string(),
        "\"string with spaces\""
    );
}

#[test]
fn parse_idents() {
    fn test_parse(input: &str, expected: &str) {
        let mut parser = Parser::new(input);
        assert_eq!(parser.parse_expression(), Expr::Ident(expected.to_string()));
    }
    test_parse("foo", "foo");
    test_parse("   Bar", "Bar");
    test_parse("baz123", "baz123");
}

#[test]
fn parse_fn_calls() {
    let mut parser = Parser::new("foo(bar, 0)");

    assert_eq!(
        parser.parse_expression(),
        Expr::FnCall {
            fn_name: "foo".to_string(),
            args: vec![Expr::Ident("bar".to_string()), Expr::Literal(Lit::Int(0))],
        }
    );

    parser = Parser::new("foo(bar(baz))");
    assert_eq!(
        parser.parse_expression(),
        Expr::FnCall {
            fn_name: "foo".to_string(),
            args: vec![Expr::FnCall {
                fn_name: "bar".to_string(),
                args: vec![Expr::Ident("baz".to_string())],
            }]
        },
    );

    parser = Parser::new("foo( )");
    assert_eq!(
        parser.parse_expression(),
        Expr::FnCall {
            fn_name: "foo".to_string(),
            args: Vec::new(),
        }
    );
}

#[test]
fn parse_arithmetic() {
    fn parse(input: &str) -> Expr {
        let mut parser = Parser::new(input);
        parser.parse_expression()
    }
    assert_eq!(parse("1 + 2").to_string(), "(1 + 2)");
    assert_eq!(parse("1 + 2 + 3").to_string(), "((1 + 2) + 3)");
    assert_eq!(parse("1 + 2 * 3").to_string(), "(1 + (2 * 3))");
    assert_eq!(parse("1 * 2 - 3").to_string(), "((1 * 2) - 3)");
    assert_eq!(parse("1 * (2 - 3)").to_string(), "(1 * (2 - 3)");
}
