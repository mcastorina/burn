use burn::lexer::Token;
use burn::parser::ast::{Expr, Item, Lit, Stmt, Type};
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
fn parse_method_calls() {
    let mut parser = Parser::new("foo.bar()");

    assert_eq!(
        parser.expression(),
        Expr::InfixOp {
            op: Token::Dot,
            lhs: Box::new(Expr::Ident("foo".to_string())),
            rhs: Box::new(Expr::FnCall {
                fn_name: "bar".to_string(),
                args: Vec::new(),
            }),
        }
    );

    parser = Parser::new("1 -> foo.bar.baz()");
    assert_eq!(
        parser.expression(),
        Expr::InfixOp {
            op: Token::Dot,
            lhs: Box::new(Expr::Ident("foo".to_string())),
            rhs: Box::new(Expr::InfixOp {
                op: Token::Dot,
                lhs: Box::new(Expr::Ident("bar".to_string())),
                rhs: Box::new(Expr::FnCall {
                    fn_name: "baz".to_string(),
                    args: vec![Expr::Literal(Lit::Int(1))],
                }),
            }),
        },
    );

    parser = Parser::new("1 -> foo::bar.baz()");
    assert_eq!(
        parser.expression(),
        Expr::InfixOp {
            op: Token::DoubleColon,
            lhs: Box::new(Expr::Ident("foo".to_string())),
            rhs: Box::new(Expr::InfixOp {
                op: Token::Dot,
                lhs: Box::new(Expr::Ident("bar".to_string())),
                rhs: Box::new(Expr::FnCall {
                    fn_name: "baz".to_string(),
                    args: vec![Expr::Literal(Lit::Int(1))],
                }),
            }),
        },
    );
}

#[test]
fn parse_dot_operator() {
    fn parse(input: &str) -> Expr {
        let mut parser = Parser::new(input);
        parser.expression()
    }
    assert_eq!(parse("foo.bar").to_string(), "(foo . bar)");
    assert_eq!(parse("foo.bar.baz").to_string(), "(foo . (bar . baz))");
    assert_eq!(parse("1.2").to_string(), "(1 . 2)");
    assert_eq!(parse("foo.bar + baz").to_string(), "((foo . bar) + baz)");
    assert_eq!(
        parse("1 + foo.bar + baz").to_string(),
        "((1 + (foo . bar)) + baz)"
    );
    assert_eq!(
        parse("1 ^ foo.bar ^ 2").to_string(),
        "(1 ^ ((foo . bar) ^ 2))"
    );
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

#[test]
fn parse_statements() {
    fn parse(input: &str) -> Stmt {
        let mut parser = Parser::new(input);
        parser.statement()
    }

    let stmt = parse(
        r#"
        {
            x := 7 + sin(y);
            {
                x = 3;
                if bar < 3 {
                    x = x + 1;
                    y = 3 * x;
                } else if bar < 2 {
                    i := 2!;
                    x = x + i;
                } else {
                    x = 1;
                }
            }
        }
    "#,
    );

    let stmts = match stmt {
        Stmt::Block { stmts } => stmts,
        _ => unreachable!(),
    };
    assert_eq!(stmts.len(), 2);

    let let_stmt = &stmts[0];
    match let_stmt {
        Stmt::Declaration { var_name, .. } => assert_eq!(var_name, "x"),
        _ => unreachable!(),
    }

    let stmts = match &stmts[1] {
        Stmt::Block { stmts } => stmts,
        _ => unreachable!(),
    };
    assert_eq!(stmts.len(), 2);

    let assignment_stmt = &stmts[0];
    match assignment_stmt {
        Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "x"),
        _ => unreachable!(),
    }

    let if_stmt = &stmts[1];
    match if_stmt {
        Stmt::IfStmt {
            condition,
            body,
            else_stmt,
        } => {
            assert!(matches!(
                condition,
                Expr::InfixOp {
                    op: Token::LeftAngleBracket,
                    lhs: _lhs,
                    rhs: _rhs,
                }
            ));
            assert_eq!(body.len(), 2);
            let x_assignment = &body[0];
            match x_assignment {
                Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "x"),
                _ => unreachable!(),
            }
            let y_assignment = &body[1];
            match y_assignment {
                Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "y"),
                _ => unreachable!(),
            }

            let else_stmt = match else_stmt {
                Some(stmt) => &**stmt,
                None => unreachable!(),
            };

            match else_stmt {
                Stmt::IfStmt {
                    condition,
                    body,
                    else_stmt,
                } => {
                    assert!(matches!(
                        condition,
                        Expr::InfixOp {
                            op: Token::LeftAngleBracket,
                            lhs: _lhs,
                            rhs: _rhs,
                        }
                    ));
                    assert_eq!(body.len(), 2);
                    let let_i = &body[0];
                    match let_i {
                        Stmt::Declaration { var_name, .. } => assert_eq!(var_name, "i"),
                        _ => unreachable!(),
                    }
                    let x_assignment = &body[1];
                    match x_assignment {
                        Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "x"),
                        _ => unreachable!(),
                    }

                    let else_stmt = match else_stmt {
                        Some(stmt) => &**stmt,
                        None => unreachable!(),
                    };

                    let stmts = match else_stmt {
                        Stmt::Block { stmts } => stmts,
                        _ => unreachable!(),
                    };
                    assert_eq!(stmts.len(), 1);

                    let x_assignment = &stmts[0];
                    match x_assignment {
                        Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "x"),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            };
        }
        _ => unreachable!(),
    }
}

#[test]
fn parse_fns() {
    fn parse(input: &str) -> Item {
        let mut parser = Parser::new(input);
        parser.item()
    }

    let func = parse(
        r#"
        fn foo(a int, b stream<u8>) stream<u8> {
            x := 1 + 2;
            yield x;
            return b;
        }
    "#,
    );

    match func {
        Item::Function {
            name,
            parameters,
            body,
            return_type,
        } => {
            assert_eq!(name, "foo");
            assert_eq!(parameters.len(), 2);
            assert_eq!(
                parameters[0],
                (
                    "a".to_string(),
                    Type {
                        name: "int".to_string(),
                        generics: Vec::new(),
                    }
                )
            );
            assert_eq!(
                parameters[1],
                (
                    "b".to_string(),
                    Type {
                        name: "stream".to_string(),
                        generics: vec![Type {
                            name: "u8".to_string(),
                            generics: Vec::new(),
                        }],
                    }
                )
            );
            assert_eq!(
                return_type,
                Some(Type {
                    name: "stream".to_string(),
                    generics: vec![Type {
                        name: "u8".to_string(),
                        generics: Vec::new(),
                    }],
                })
            );
            assert_eq!(body.len(), 3);
            assert_eq!(
                body[1],
                Stmt::YieldStmt {
                    value: Expr::Ident("x".to_string()),
                }
            );
            assert_eq!(
                body[2],
                Stmt::ReturnStmt {
                    value: Some(Expr::Ident("b".to_string())),
                }
            );
        }
        _ => unreachable!(),
    }
}

#[test]
fn parse_import() {
    fn parse(input: &str) -> Item {
        let mut parser = Parser::new(input);
        parser.item()
    }
    assert_eq!(parse("import foo;"), Item::Import(vec!["foo".to_string()]));
    assert_eq!(
        parse("import foo::bar;"),
        Item::Import(vec!["foo".to_string(), "bar".to_string()])
    );
}

#[test]
fn parse_for() {
    fn parse(input: &str) -> Stmt {
        let mut parser = Parser::new(input);
        parser.statement()
    }
    assert_eq!(
        parse("for foo in bar { yield foo; }"),
        Stmt::ForLoop {
            var_name: "foo".to_string(),
            stream: Expr::Ident("bar".to_string()),
            stmts: vec![Stmt::YieldStmt {
                value: Expr::Ident("foo".to_string())
            }],
        }
    );
    assert_eq!(
        parse("for foo in bar() {}"),
        Stmt::ForLoop {
            var_name: "foo".to_string(),
            stream: Expr::FnCall {
                fn_name: "bar".to_string(),
                args: Vec::new(),
            },
            stmts: Vec::new(),
        }
    );
}
