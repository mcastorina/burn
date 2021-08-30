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
                        let value = self.expression();
                        self.consume(Token::Semicolon);
                        ast::Stmt::Declaration {
                            var_name: ident.to_string(),
                            value,
                        }
                    }
                    op @ Token::Assign => {
                        self.consume(op);
                        let value = self.expression();
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
                    condition: condition,
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
            Token::KeywordReturn => {
                self.consume(Token::KeywordReturn);
                if self.at(Token::Semicolon) {
                    self.consume(Token::Semicolon);
                    ast::Stmt::ReturnStmt { value: None }
                } else {
                    let expr = self.expression();
                    self.consume(Token::Semicolon);
                    ast::Stmt::ReturnStmt { value: Some(expr) }
                }
            }
            Token::KeywordYield => {
                self.consume(Token::KeywordYield);
                let expr = self.expression();
                self.consume(Token::Semicolon);
                ast::Stmt::YieldStmt { value: expr }
            }
            Token::KeywordFor => {
                self.consume(Token::KeywordFor);
                let (ident_tok, ident_name) = self
                    .next()
                    .expect("Tried to parse identifier, but there were no more tokens");
                assert_eq!(
                    ident_tok,
                    Token::Ident,
                    "Expected identifier at start of type, but found `{}`",
                    ident_tok
                );
                let (in_tok, _) = self
                    .next()
                    .expect("Tried to parse `in`, but there were no more tokens");
                assert_eq!(
                    in_tok,
                    Token::KeywordIn,
                    "Expected `in` after identifier, but found `{}`",
                    in_tok
                );
                let stream = self.expression();
                assert!(
                    self.at(Token::LeftCurlyBracket),
                    "Expected block after for header"
                );
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
            _ => unreachable!(),
        }
    }

    pub fn type_(&mut self) -> ast::Type {
        let (ident, name) = self
            .next()
            .expect("Tried to parse type, but there were no more tokens");
        assert_eq!(
            ident,
            Token::Ident,
            "Expected identifier at start of type, but found `{}`",
            ident
        );
        let mut generics = Vec::new();
        if self.at(Token::LeftAngleBracket) {
            self.consume(Token::LeftAngleBracket);
            while !self.at(Token::RightAngleBracket) {
                generics.push(self.type_());
                if self.at(Token::Comma) {
                    self.consume(Token::Comma);
                }
            }
            self.consume(Token::RightAngleBracket);
        }
        ast::Type {
            name: name.to_string(),
            generics,
        }
    }

    pub fn item(&mut self) -> ast::Item {
        let mut parameters = Vec::new();
        match self.peek() {
            Token::KeywordFn => {
                self.consume(Token::KeywordFn);
                let (ident, name) = self
                    .next()
                    .expect("Tried to parse function name, but there were no more tokens");
                assert_eq!(
                    ident,
                    Token::Ident,
                    "Expected identifier as function name, but found `{}`",
                    ident
                );
                self.consume(Token::LeftParen);
                while !self.at(Token::RightParen) {
                    let (param, param_name) = self
                        .next()
                        .expect("Tried to parse function parameter, but there were no more tokens");
                    assert_eq!(
                        param,
                        Token::Ident,
                        "Expected identifier as function parameter, but found `{}`",
                        param
                    );
                    let param_type = self.type_();
                    parameters.push((param_name.to_string(), param_type));
                    if self.at(Token::Comma) {
                        self.consume(Token::Comma);
                    }
                }
                self.consume(Token::RightParen);
                assert!(
                    self.at(Token::LeftCurlyBracket) || self.at(Token::Ident),
                    "Expected block or return type after function header"
                );
                let return_type = if self.at(Token::Ident) {
                    Some(self.type_())
                } else {
                    None
                };
                assert!(
                    self.at(Token::LeftCurlyBracket),
                    "Expected block after function header"
                );
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
            Token::KeywordImport => {
                self.consume(Token::KeywordImport);
                let (ident_tok, ident_name) = self
                    .next()
                    .expect("Expected an identifier after `import`, but there were no more tokens");
                assert_eq!(
                    ident_tok,
                    Token::Ident,
                    "Expected an identifier after `import`, but found `{}`",
                    ident_name
                );
                let mut idents = vec![ident_name.to_string()];
                while self.peek() == Token::DoubleColon {
                    self.consume(Token::DoubleColon);
                    let (ident_tok, ident_name) = self.next().expect(
                        "Expected an identifier after `import`, but there were no more tokens",
                    );
                    assert_eq!(
                        ident_tok,
                        Token::Ident,
                        "Expected an identifier after `import`, but found `{}`",
                        ident_name
                    );
                    idents.push(ident_name.to_string());
                }
                self.consume(Token::Semicolon);
                ast::Item::Import(idents)
            }
            _ => unreachable!(),
        }
    }
}
