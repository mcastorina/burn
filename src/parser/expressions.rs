use super::ast;
use super::Parser;
use crate::{lexer::Token, T};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = (Token, std::ops::Range<usize>)>,
{
    pub fn expression(&mut self) -> ast::Expr {
        self.parse_expression(0)
    }
    fn parse_expression(&mut self, binding_power: u8) -> ast::Expr {
        let mut lhs = match self.peek() {
            T![num(_)] | T![string] | T![byte] => {
                let (literal_token, literal_text) = self.next().unwrap();
                let lit = match literal_token {
                    T![num(n)] => ast::Lit::Int(n),
                    T![string] => {
                        ast::Lit::Str(literal_text[1..literal_text.len() - 1].to_string())
                    }
                    T![byte] => ast::Lit::Byt(literal_text[1..literal_text.len() - 1].to_string()),
                    _ => unreachable!(),
                };
                ast::Expr::Literal(lit)
            }
            T![ident] => {
                let (_, ident_name) = self.next().unwrap();
                if !self.at(T!['(']) {
                    ast::Expr::Ident(ident_name.to_string())
                } else {
                    // function call
                    let mut args = Vec::new();
                    self.consume(T!['(']);
                    while !self.at(T![')']) {
                        let arg = self.parse_expression(0);
                        args.push(arg);
                        if self.at(T![,]) {
                            self.consume(T![,]);
                        } else if !self.at(T![')']) {
                            panic!("Unexpected token");
                        }
                    }
                    self.consume(T![')']);
                    ast::Expr::FnCall {
                        fn_name: ident_name.to_string(),
                        args,
                    }
                }
            }
            T!['('] => {
                self.consume(T!['(']);
                let expr = self.parse_expression(0);
                self.consume(T![')']);
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
                op @ T![+]
                | op @ T![-]
                | op @ T![*]
                | op @ T![/]
                | op @ T![^]
                | op @ T![==]
                | op @ T![!=]
                | op @ T![&&]
                | op @ T![||]
                | op @ T![<]
                | op @ T![<=]
                | op @ T![>]
                | op @ T![>=]
                | op @ T![!]
                | op @ T![.]
                | op @ T![::]
                | op @ T![->] => op,
                T![eof] | T![')'] | T!['}'] | T!['{'] | T![,] | T![;] => break,
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
                if op == T![->] {
                    if let ast::Expr::FnCall {
                        fn_name: _,
                        ref mut args,
                    } = rhs
                    {
                        args.push(lhs);
                        lhs = rhs;
                    } else {
                        // could be a FnCall after a dot / double colon operator
                        let mut expr = &mut rhs;
                        loop {
                            match expr {
                                ast::Expr::InfixOp {
                                    op: T![.],
                                    lhs: _,
                                    rhs,
                                }
                                | ast::Expr::InfixOp {
                                    op: T![::],
                                    lhs: _,
                                    rhs,
                                } => {
                                    expr = rhs;
                                }
                                _ => {
                                    panic!(
                                        "Expected a function call after the arrow operator, found `{}`",
                                        rhs,
                                    );
                                }
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
            T![+] | T![-] | T![!] => ((), 51),
            _ => unreachable!("Not a prefix operator: `{:?}`", self),
        }
    }
    fn infix_binding_power(&self) -> Option<(u8, u8)> {
        let result = match self {
            T![||] => (1, 2),
            T![&&] => (3, 4),
            T![==] | T![!=] => (5, 6),
            T![<] | T![>] | T![<=] | T![>=] => (7, 8),
            T![->] => (9, 10),
            T![+] | T![-] => (11, 12),
            T![*] | T![/] => (13, 14),
            T![.] | T![::] => (24, 23),
            _ => return None,
        };
        Some(result)
    }
    fn postfix_binding_power(&self) -> Option<(u8, ())> {
        let result = match self {
            T![!] => (101, ()),
            _ => return None,
        };
        Some(result)
    }
}
