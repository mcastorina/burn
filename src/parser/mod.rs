pub mod ast;
mod expressions;

use crate::lexer::Token;
use logos::{Logos, SpannedIter};
use std::iter::Peekable;

pub struct Parser<'input, I>
where
    I: Iterator<Item = (Token, std::ops::Range<usize>)>,
{
    input: &'input str,
    tokens: Peekable<I>,
}

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = (Token, std::ops::Range<usize>)>,
{
    pub fn peek(&mut self) -> Token {
        self.tokens
            .peek()
            .map(|(token, _)| *token)
            .unwrap_or(Token::EOF)
    }
    pub fn at(&mut self, kind: Token) -> bool {
        self.peek() == kind
    }
    pub fn next(&mut self) -> Option<(Token, &'input str)> {
        self.tokens.next().map(|(token, r)| (token, &self.input[r]))
    }
    pub fn consume(&mut self, expected: Token) {
        let (token, _) = self.next().expect(&format!(
            "Expected to consume `{}`, but there was no next token",
            expected
        ));
        assert_eq!(
            token, expected,
            "Expected to consume `{}`, but found `{}`",
            expected, token
        );
    }
}

impl<'input> Parser<'input, SpannedIter<'input, Token>> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            tokens: Token::lexer(input).spanned().peekable(),
        }
    }
}
