use super::ast;
use super::Parser;
use crate::lexer::Token;

// TODO:
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = (Token, std::ops::Range<usize>)>,
{
    pub fn parse_expression(&mut self) -> ast::Expr {
        let mut lhs = match self.peek() {
            Token::Number(_) | Token::String => {
                let (literal_token, literal_text) = self.next().unwrap();
                let lit = match literal_token {
                    Token::Number(n) => ast::Lit::Int(n),
                    Token::String => {
                        ast::Lit::Str(literal_text[1..literal_text.len() - 1].to_string())
                    }
                    _ => unreachable!(),
                };
                ast::Expr::Literal(lit)
            }
            Token::Ident => {
                let (_, ident_name) = self.next().unwrap();
                if !self.at(Token::LeftParen) {
                    return ast::Expr::Ident(ident_name.to_string());
                }
                // function call
                let mut args = Vec::new();
                self.consume(Token::LeftParen);
                while !self.at(Token::RightParen) {
                    let arg = self.parse_expression();
                    args.push(arg);
                    if self.at(Token::Comma) {
                        self.consume(Token::Comma);
                    } else if !self.at(Token::RightParen) {
                        panic!("Unexpected token");
                    }
                }
                self.consume(Token::RightParen);
                ast::Expr::FnCall {
                    fn_name: ident_name.to_string(),
                    args,
                }
            }
            Token::LeftParen => {
                self.consume(Token::LeftParen);
                let expr = self.parse_expression();
                self.consume(Token::RightParen);
                expr
            }
            op @ Token::Plus | op @ Token::Minus | op @ Token::Bang => {
                self.consume(op);
                let expr = self.parse_expression();
                ast::Expr::PrefixOp {
                    op,
                    expr: Box::new(expr),
                }
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        };

        loop {
            let op = match self.peek() {
                op @ Token::Plus
                | op @ Token::Minus
                | op @ Token::Asterisk
                | op @ Token::Slash
                | op @ Token::Caret
                | op @ Token::Equal
                | op @ Token::NotEqual
                | op @ Token::And
                | op @ Token::Or
                | op @ Token::LeftAngleBracket
                | op @ Token::LessOrEqual
                | op @ Token::RightAngleBracket
                | op @ Token::GreaterOrEqual
                | op @ Token::Bang => op,
                Token::EOF
                | Token::RightParen
                | Token::RightCurlyBracket
                | Token::Comma
                | Token::Semicolon => break,
                kind => panic!("Unknown operator: `{}`", kind),
            };
            self.consume(op);
            let rhs = self.parse_expression();
            lhs = ast::Expr::InfixOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        lhs
    }
}
