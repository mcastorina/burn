use super::ast;
use super::Parser;
use crate::{lexer::Token, T};
use ast::Expr;

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = (Token, std::ops::Range<usize>)>,
{
    pub fn expression(&mut self) -> Expr {
        self.parse_expression(0)
    }
    // TODO:
    // * true recursive descent
    // * refactor common operations
    // * define context free grammar
    fn parse_expression(&mut self, binding_power: u8) -> Expr {
        let mut lhs = match self.peek() {
            T![num(_)] | T![string] | T![byte] => self.literal(),
            T![ident] => {
                let (tok, ident_name) = self.next().unwrap();
                if !self.at(T!['(']) {
                    Expr::Ident(ident_name.to_string())
                } else {
                    self.push((tok, ident_name));
                    self.fn_call()
                }
            }
            T!['('] => {
                let tup = self.tuple();
                // tuple with one expression is just the expression
                // this is needed for arithmetic extension
                if let Expr::Tuple(mut vec) = tup {
                    if vec.len() == 1 {
                        vec.swap_remove(0)
                    } else {
                        Expr::Tuple(vec)
                    }
                } else {
                    unreachable!()
                }
            }
            op @ Token::Plus | op @ Token::Minus | op @ Token::Bang => {
                self.consume(op);
                let (_, right_binding_power) = op.prefix_binding_power();
                let expr = self.parse_expression(right_binding_power);
                Expr::PrefixOp {
                    op,
                    expr: Box::new(expr),
                }
            }
            T![_] => {
                self.consume(T![_]);
                Expr::Placeholder
            }
            kind => panic!("Unknown start of expression: `{}`", kind),
        };

        // extend lhs expression
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
                lhs = Expr::PostfixOp {
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
                if op != T![->] || matches!(rhs, Expr::Ident(_)) {
                    lhs = Expr::InfixOp {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                    continue;
                }
                if let Expr::FnCall {
                    fn_name: _,
                    ref mut args,
                } = rhs
                {
                    let lhs_args = if let Expr::Tuple(items) = lhs {
                        items
                    } else {
                        vec![lhs]
                    };
                    for arg in lhs_args {
                        // try replacing the first Expr::Placeholder, otherwise append to args
                        if let Some(index) = args.iter().position(|a| *a == Expr::Placeholder) {
                            args[index] = arg;
                        } else {
                            args.push(arg);
                        }
                    }
                    lhs = rhs;
                } else {
                    // could be a FnCall after a dot / double colon operator
                    let mut expr = &mut rhs;
                    loop {
                        if let Expr::InfixOp { op, rhs, .. } = expr {
                            if *op != T![.] && *op != T![::] {
                                panic!(
                                    "Expected a function call after the arrow operator, found `{}`",
                                    rhs,
                                );
                            }
                            expr = rhs;
                        }
                        if let Expr::FnCall {
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
                continue;
            }
            break;
        }

        lhs
    }

    fn literal(&mut self) -> Expr {
        let (literal_token, literal_text) = self.next().unwrap();
        let lit = match literal_token {
            T![num(n)] => ast::Lit::Int(n),
            T![string] => ast::Lit::Str(literal_text[1..literal_text.len() - 1].to_string()),
            T![byte] => ast::Lit::Byt(literal_text[1..literal_text.len() - 1].to_string()),
            tok => panic!("Unexpected literal token: {:?}", tok),
        };
        Expr::Literal(lit)
    }
    fn fn_call(&mut self) -> Expr {
        let (_, ident_name) = self.next().unwrap();
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
        Expr::FnCall {
            fn_name: ident_name.to_string(),
            args,
        }
    }
    fn tuple(&mut self) -> Expr {
        self.consume(T!['(']);
        let mut args = vec![];
        while !self.at(T![')']) {
            args.push(self.parse_expression(0));
            if self.at(T![,]) {
                self.consume(T![,]);
            } else if !self.at(T![')']) {
                panic!("Unexpected token");
            }
        }
        self.consume(T![')']);
        Expr::Tuple(args)
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
