pub mod ast;
mod expressions;
mod hierarchy;

use crate::lexer::Token;
use logos::{Logos, SpannedIter};
use std::iter::Peekable;

pub struct Parser<'input, I>
where
    I: Iterator<Item = (Token, std::ops::Range<usize>)>,
{
    input: &'input str,
    tokens: Peekable<I>,
    reserve: Option<(Token, &'input str)>,
}

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = (Token, std::ops::Range<usize>)>,
{
    pub fn peek(&mut self) -> Token {
        match self.reserve {
            Some((tok, _)) => tok,
            None => self
                .tokens
                .peek()
                .map(|(token, _)| *token)
                .unwrap_or(Token::EOF),
        }
    }
    pub fn at(&mut self, kind: Token) -> bool {
        self.peek() == kind
    }
    pub fn next(&mut self) -> Option<(Token, &'input str)> {
        match self.reserve {
            Some(data) => {
                self.reserve = None;
                Some(data)
            }
            None => self.tokens.next().map(|(token, r)| (token, &self.input[r])),
        }
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
    pub fn push(&mut self, data: (Token, &'input str)) {
        if self.reserve.is_some() {
            panic!("Cannot push; reserve is full: {:?}", self.reserve);
        }
        self.reserve = Some(data);
    }
}

impl<'input> Parser<'input, SpannedIter<'input, Token>> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            tokens: Token::lexer(input).spanned().peekable(),
            reserve: None,
        }
    }
}
