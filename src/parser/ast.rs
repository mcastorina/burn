use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Lit),
    Ident(String),
    FnCall {
        fn_name: String,
        args: Vec<Expr>,
    },
    PrefixOp {
        op: Token,
        expr: Box<Expr>,
    },
    InfixOp {
        op: Token,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    PostfixOp {
        op: Token,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(usize),
    Str(String),
    Byt(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Declaration {
        var_name: String,
        value: Expr,
    },
    Assignment {
        var_name: String,
        value: Expr,
    },
    IfStmt {
        condition: Expr,
        body: Vec<Stmt>,
        else_stmt: Option<Box<Stmt>>,
    },
    ReturnStmt {
        value: Option<Expr>,
    },
    YieldStmt {
        value: Expr,
    },
    Block {
        stmts: Vec<Stmt>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function {
        name: String,
        parameters: Vec<(String, Type)>,
        body: Vec<Stmt>,
        return_type: Option<Type>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub name: String,
    pub generics: Vec<Type>,
}

use std::fmt::{Display, Error, Formatter};
impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Expr::Literal(lit) => write!(f, "{}", lit),
            Expr::Ident(ident) => write!(f, "{}", ident),
            Expr::FnCall { fn_name, args } => write!(
                f,
                "{}({})",
                fn_name,
                args.into_iter()
                    .map(|e| e.to_string())
                    .fold(String::new(), |s, e| {
                        if s.len() == 0 {
                            e
                        } else {
                            s + ", " + &e
                        }
                    })
            ),
            Expr::PrefixOp { op, expr } => write!(f, "({}{})", op, expr),
            Expr::InfixOp { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::PostfixOp { op, expr } => write!(f, "({}{})", expr, op),
        }
    }
}

impl Display for Lit {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Lit::Int(n) => write!(f, "{}", n),
            Lit::Str(s) => write!(f, "\"{}\"", s),
            Lit::Byt(s) => write!(f, "`{}`", s),
        }
    }
}

#[test]
fn display() {
    assert_eq!(Expr::Literal(Lit::Int(1)).to_string(), "1");
    assert_eq!(
        Expr::Literal(Lit::Str("foo".to_string())).to_string(),
        "\"foo\""
    );
    assert_eq!(
        Expr::InfixOp {
            op: Token::Plus,
            lhs: Box::new(Expr::Literal(Lit::Int(1))),
            rhs: Box::new(Expr::Literal(Lit::Int(2))),
        }
        .to_string(),
        "(1 + 2)"
    );
    assert_eq!(Expr::Literal(Lit::Byt("a".to_string())).to_string(), "`a`");
}
