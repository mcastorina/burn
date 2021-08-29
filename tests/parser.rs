use burn::parser::ast::{Expr, Lit};
use burn::parser::Parser;

#[test]
fn parse_literals() {
    fn parse(input: &str) -> Expr {
        let mut parser = Parser::new(input);
        parser.expression()
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
        assert_eq!(parser.expression(), Expr::Ident(expected.to_string()));
    }
    test_parse("foo", "foo");
    test_parse("   Bar", "Bar");
    test_parse("baz123", "baz123");
}

#[test]
fn parse_fn_calls() {
    let mut parser = Parser::new("foo(bar, 0)");

    assert_eq!(
        parser.expression(),
        Expr::FnCall {
            fn_name: "foo".to_string(),
            args: vec![Expr::Ident("bar".to_string()), Expr::Literal(Lit::Int(0))],
        }
    );

    parser = Parser::new("foo(bar(baz))");
    assert_eq!(
        parser.expression(),
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
        parser.expression(),
        Expr::FnCall {
            fn_name: "foo".to_string(),
            args: Vec::new(),
        }
    );

    parser = Parser::new("foo -> bar()");
    assert_eq!(
        parser.expression(),
        Expr::FnCall {
            fn_name: "bar".to_string(),
            args: vec![Expr::Ident("foo".to_string())],
        }
    );

    parser = Parser::new("foo() -> bar(\"baz\")");
    assert_eq!(
        parser.expression(),
        Expr::FnCall {
            fn_name: "bar".to_string(),
            args: vec![
                Expr::Literal(Lit::Str("baz".to_string())),
                Expr::FnCall {
                    fn_name: "foo".to_string(),
                    args: Vec::new(),
                }
            ],
        }
    );

    parser = Parser::new("1+2*3 -> foo()");
    assert_eq!(parser.expression().to_string(), "foo((1 + (2 * 3)))");

    parser = Parser::new("foo() -> bar() -> baz()");
    assert_eq!(parser.expression().to_string(), "baz(bar(foo()))");

    parser = Parser::new("foo() -> bar() -> baz() -> buzz()");
    assert_eq!(parser.expression().to_string(), "buzz(baz(bar(foo())))");
}

#[test]
fn parse_arithmetic() {
    fn parse(input: &str) -> Expr {
        let mut parser = Parser::new(input);
        parser.expression()
    }
    assert_eq!(parse("1 + 2").to_string(), "(1 + 2)");
    assert_eq!(parse("1 + 2 + 3").to_string(), "((1 + 2) + 3)");
    assert_eq!(parse("1 + 2 * 3").to_string(), "(1 + (2 * 3))");
    assert_eq!(parse("1 * 2 - 3").to_string(), "((1 * 2) - 3)");
    assert_eq!(parse("1*(2-3)").to_string(), "(1 * (2 - 3))");
    assert_eq!(parse("-10 + 4").to_string(), "((-10) + 4)");
    assert_eq!(parse("4 * -10").to_string(), "(4 * (-10))");
    assert_eq!(parse("1 + 2!").to_string(), "(1 + (2!))");
}

#[test]
fn parse_binary_expressions() {
    fn parse(input: &str) -> Expr {
        let mut parser = Parser::new(input);
        parser.expression()
    }

    assert_eq!(
        parse(r#"45 + 3 + 5 * 4^8^9 / 6 > 4 && test - 7 / 4 == "Hallo""#).to_string(),
        r#"((((45 + 3) + ((5 * (4 ^ (8 ^ 9))) / 6)) > 4) && ((test - (7 / 4)) == "Hallo"))"#
    );
    assert_eq!(parse("1 + 2 == 3 + 4").to_string(), "((1 + 2) == (3 + 4))");
    assert_eq!(parse("1 < 2 == 3 > 4").to_string(), "((1 < 2) == (3 > 4))");
    assert_eq!(parse("1 -> foo() < -10").to_string(), "(foo(1) < (-10))");
    assert_eq!(
        parse("1 -> foo() == 2 -> bar()").to_string(),
        "(foo(1) == bar(2))"
    );
}
