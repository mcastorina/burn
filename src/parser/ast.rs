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
}

use std::fmt::{Display, Error, Formatter};
impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        todo!()
    }
}
