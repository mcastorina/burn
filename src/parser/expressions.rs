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
    pub fn expression(&mut self) -> ast::Expr {
        self.parse_expression(0)
    }
    fn parse_expression(&mut self, binding_power: u8) -> ast::Expr {
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
                    ast::Expr::Ident(ident_name.to_string())
                } else {
                    // function call
                    let mut args = Vec::new();
                    self.consume(Token::LeftParen);
                    while !self.at(Token::RightParen) {
                        let arg = self.parse_expression(0);
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
            }
            Token::LeftParen => {
                self.consume(Token::LeftParen);
                let expr = self.parse_expression(0);
                self.consume(Token::RightParen);
                expr
            }
            op @ Token::Plus | op @ Token::Minus | op @ Token::Bang => {
                self.consume(op);
                let (_, right_binding_power) = op.prefix_binding_power();
                let expr = self.parse_expression(right_binding_power);
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
                | op @ Token::Bang
                | op @ Token::Dot
                | op @ Token::RightArrow => op,
                Token::EOF
                | Token::RightParen
                | Token::RightCurlyBracket
                | Token::LeftCurlyBracket
                | Token::Comma
                | Token::Semicolon => break,
                kind => panic!("Unknown operator: `{}`", kind),
            };

            if let Some((left_bp, _)) = op.postfix_binding_power() {
                if left_bp < binding_power {
                    break;
                }
                self.consume(op);
                lhs = ast::Expr::PostfixOp {
                    op,
                    expr: Box::new(lhs),
                };
                continue;
            }

            if let Some((left_bp, right_bp)) = op.infix_binding_power() {
                if left_bp < binding_power {
                    break;
                }
                self.consume(op);
                let mut rhs = self.parse_expression(right_bp);
                if op == Token::RightArrow {
                    if let ast::Expr::FnCall {
                        fn_name: _,
                        ref mut args,
                    } = rhs
                    {
                        args.push(lhs);
                        lhs = rhs;
                    } else {
                        // could be a FnCall after a dot operator
                        let mut expr = &mut rhs;
                        loop {
                            if let ast::Expr::InfixOp {
                                op: Token::Dot,
                                lhs: _,
                                rhs,
                            } = expr
                            {
                                expr = rhs;
                            } else {
                                panic!(
                                    "Expected a function call after the arrow operator, found `{}`",
                                    rhs,
                                );
                            }
                            if let ast::Expr::FnCall {
                                fn_name: _,
                                ref mut args,
                            } = expr
                            {
                                args.push(lhs);
                                lhs = rhs;
                                break;
                            }
                        }
                    }
                } else {
                    lhs = ast::Expr::InfixOp {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                continue;
            }
            break;
        }

        lhs
    }
}

trait Operator {
    fn prefix_binding_power(&self) -> ((), u8);
    fn infix_binding_power(&self) -> Option<(u8, u8)>;
    fn postfix_binding_power(&self) -> Option<(u8, ())>;
}

impl Operator for Token {
    fn prefix_binding_power(&self) -> ((), u8) {
        match self {
            Token::Plus | Token::Minus | Token::Bang => ((), 51),
            _ => unreachable!("Not a prefix operator: `{:?}`", self),
        }
    }
    fn infix_binding_power(&self) -> Option<(u8, u8)> {
        let result = match self {
            Token::Or => (1, 2),
            Token::And => (3, 4),
            Token::Equal | Token::NotEqual => (5, 6),
            Token::LeftAngleBracket
            | Token::RightAngleBracket
            | Token::LessOrEqual
            | Token::GreaterOrEqual => (7, 8),
            Token::RightArrow => (9, 10),
            Token::Plus | Token::Minus => (11, 12),
            Token::Asterisk | Token::Slash => (13, 14),
            Token::Caret => (22, 21), // <- This binds stronger to the left!
            Token::Dot => (24, 23),
            _ => return None,
        };
        Some(result)
    }
    fn postfix_binding_power(&self) -> Option<(u8, ())> {
        let result = match self {
            Token::Bang => (101, ()),
            _ => return None,
        };
        Some(result)
    }
}
