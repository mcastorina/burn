use super::ast;
use super::Parser;
use crate::{lexer::Token, T};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = (Token, std::ops::Range<usize>)>,
{
    pub fn statement(&mut self) -> ast::Stmt {
        match self.peek() {
            T![ident] => {
                let (_, ident) = self.next().unwrap();
                match self.peek() {
                    op @ T![:=] => {
                        self.consume(op);
                        let value = self.expression();
                        self.consume(T![;]);
                        ast::Stmt::Declaration {
                            var_name: ident.to_string(),
                            value,
                        }
                    }
                    op @ T![=] => {
                        self.consume(op);
                        let value = self.expression();
                        self.consume(T![;]);
                        ast::Stmt::Assignment {
                            var_name: ident.to_string(),
                            value,
                        }
                    }
                    _ => {
                        self.push((T![ident], ident));
                        let expr = self.expression();
                        self.consume(T![;]);
                        ast::Stmt::Expr(expr)
                    }
                }
            }
            T![if] => {
                self.consume(T![if]);
                let condition = self.expression();
                assert!(self.at(T!['{']), "Expected a block after `if` statement");
                let body = match self.statement() {
                    ast::Stmt::Block { stmts } => stmts,
                    _ => unreachable!(),
                };

                let else_stmt = if self.at(T![else]) {
                    self.consume(T![else]);
                    assert!(
                        self.at(T![if]) || self.at(T!['{']),
                        "Expected a block or an `if` after `else` statement"
                    );
                    Some(Box::new(self.statement()))
                } else {
                    None
                };
                ast::Stmt::IfStmt {
                    condition: condition,
                    body,
                    else_stmt,
                }
            }
            T!['{'] => {
                self.consume(T!['{']);
                let mut stmts = Vec::new();
                while !self.at(T!['}']) {
                    stmts.push(self.statement());
                }
                self.consume(T!['}']);
                ast::Stmt::Block { stmts }
            }
            T![return] => {
                self.consume(T![return]);
                if self.at(T![;]) {
                    self.consume(T![;]);
                    ast::Stmt::ReturnStmt { value: None }
                } else {
                    let expr = self.expression();
                    self.consume(T![;]);
                    ast::Stmt::ReturnStmt { value: Some(expr) }
                }
            }
            T![continue] => {
                self.consume(T![continue]);
                self.consume(T![;]);
                ast::Stmt::ContinueStmt
            }
            T![for] => {
                self.consume(T![for]);
                let (ident_tok, ident_name) = self
                    .next()
                    .expect("Tried to parse identifier, but there were no more tokens");
                assert_eq!(
                    ident_tok,
                    T![ident],
                    "Expected identifier at start of type, but found `{}`",
                    ident_tok
                );
                self.consume(T![in]);
                let stream = self.expression();
                assert!(self.at(T!['{']), "Expected block after for header");
                let body = match self.statement() {
                    ast::Stmt::Block { stmts } => stmts,
                    _ => unreachable!(),
                };
                ast::Stmt::ForLoop {
                    var_name: ident_name.to_string(),
                    stream: stream,
                    stmts: body,
                }
            }
            _ => {
                let expr = self.expression();
                self.consume(T![;]);
                ast::Stmt::Expr(expr)
            }
        }
    }

    pub fn type_(&mut self) -> ast::Type {
        let (ident, name) = self
            .next()
            .expect("Tried to parse type, but there were no more tokens");
        assert_eq!(
            ident,
            T![ident],
            "Expected identifier at start of type, but found `{}`",
            ident
        );
        let mut generics = Vec::new();
        if self.at(T![<]) {
            self.consume(T![<]);
            while !self.at(T![>]) {
                generics.push(self.type_());
                if self.at(T![,]) {
                    self.consume(T![,]);
                }
            }
            self.consume(T![>]);
        }
        ast::Type {
            name: name.to_string(),
            generics,
        }
    }

    pub fn item(&mut self) -> ast::Item {
        let mut parameters = Vec::new();
        match self.peek() {
            T![fn] => {
                self.consume(T![fn]);
                let (ident, name) = self
                    .next()
                    .expect("Tried to parse function name, but there were no more tokens");
                assert_eq!(
                    ident,
                    T![ident],
                    "Expected identifier as function name, but found `{}`",
                    ident
                );
                self.consume(T!['(']);
                while !self.at(T![')']) {
                    let (param, param_name) = self
                        .next()
                        .expect("Tried to parse function parameter, but there were no more tokens");
                    assert_eq!(
                        param,
                        T![ident],
                        "Expected identifier as function parameter, but found `{}`",
                        param
                    );
                    let param_type = self.type_();
                    parameters.push((param_name.to_string(), param_type));
                    if self.at(T![,]) {
                        self.consume(T![,]);
                    }
                }
                self.consume(T![')']);
                assert!(
                    self.at(T!['{']) || self.at(T![ident]),
                    "Expected block or return type after function header"
                );
                let return_type = if self.at(T![ident]) {
                    Some(self.type_())
                } else {
                    None
                };
                assert!(self.at(T!['{']), "Expected block after function header");
                let body = match self.statement() {
                    ast::Stmt::Block { stmts } => stmts,
                    _ => unreachable!(),
                };
                ast::Item::Function {
                    name: name.to_string(),
                    parameters,
                    body,
                    return_type,
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn file(&mut self) -> Vec<ast::Item> {
        let mut items = Vec::new();
        while !self.at(T![eof]) {
            let item = self.item();
            items.push(item);
        }
        items
    }
}
