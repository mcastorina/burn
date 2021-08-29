use super::ast;
use super::Parser;
use crate::lexer::Token;

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = (Token, std::ops::Range<usize>)>,
{
    pub fn statement(&mut self) -> ast::Stmt {
        match self.peek() {
            Token::Ident => {
                let (_, ident) = self.next().unwrap();
                match self.peek() {
                    op @ Token::Declare => {
                        self.consume(op);
                        let value = Box::new(self.expression());
                        self.consume(Token::Semicolon);
                        ast::Stmt::Declaration {
                            var_name: ident.to_string(),
                            value,
                        }
                    }
                    op @ Token::Assign => {
                        self.consume(op);
                        let value = Box::new(self.expression());
                        self.consume(Token::Semicolon);
                        ast::Stmt::Assignment {
                            var_name: ident.to_string(),
                            value,
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Token::KeywordIf => {
                self.consume(Token::KeywordIf);
                let condition = self.expression();
                assert!(
                    self.at(Token::LeftCurlyBracket),
                    "Expected a block after `if` statement"
                );
                let body = match self.statement() {
                    ast::Stmt::Block { stmts } => stmts,
                    _ => unreachable!(),
                };

                let else_stmt = if self.at(Token::KeywordElse) {
                    self.consume(Token::KeywordElse);
                    assert!(
                        self.at(Token::KeywordIf) || self.at(Token::LeftCurlyBracket),
                        "Expected a block or an `if` after `else` statement"
                    );
                    Some(Box::new(self.statement()))
                } else {
                    None
                };
                ast::Stmt::IfStmt {
                    condition: Box::new(condition),
                    body,
                    else_stmt,
                }
            }
            Token::LeftCurlyBracket => {
                self.consume(Token::LeftCurlyBracket);
                let mut stmts = Vec::new();
                while !self.at(Token::RightCurlyBracket) {
                    stmts.push(self.statement());
                }
                self.consume(Token::RightCurlyBracket);
                ast::Stmt::Block { stmts }
            }
            _ => unreachable!(),
        }
    }
}
