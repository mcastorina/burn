pub mod ast;
mod expressions;

use crate::lexer::Token;
use logos::{Lexer, Logos};

pub struct Parser<'input> {
    tokens: Lexer<'input, Token>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            tokens: Token::lexer(input),
        }
    }
}
