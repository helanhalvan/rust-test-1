use std::iter::FromIterator;

use crate::function::{self, Data, ProgramState};
use crate::segments::{self, Clause, Segment};
use crate::tokens::{self, Token};

//TODO we will probably need a lot of expr subtypes
#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(Vec<char>),
    Constant(function::Data),
    ADD(Box<Expr>, Box<Expr>),
    Call(Vec<char>, Vec<Expr>),
}

pub fn eval(c: function::Program, p: function::ProgramState, expr: Expr) -> function::Data {
    match expr.clone() {
        Expr::Call(f, args) => {
            let args1 = args
                .into_iter()
                .map(|arg| eval(c.clone(), p.clone(), arg))
                .collect();
            return function::call(c, f, args1);
        }
        Expr::Constant(c) => {
            return c;
        }
        Expr::Identifier(l) => match p.get(&l) {
            Some(v) => return v.clone(),
            _ => {
                println!("UNBOUND VAR{:#?}{:#?}{:#?}\n", p, expr, l);
                unimplemented!()
            }
        },
        Expr::ADD(l, r) => {
            let l1 = eval(c.clone(), p.clone(), *l);
            let r1 = eval(c.clone(), p.clone(), *r);
            match (l1.clone(), r1.clone()) {
                (function::Data::Number(l2), function::Data::Number(r2)) => {
                    return Data::Number(l2 + r2);
                }
                _ => {
                    println!("NOT NUMBERS{:#?}{:#?}{:#?}{:#?}{:#?}\n", c, p, expr, l1, r1);
                    unimplemented!()
                }
            }
        }
        _ => {
            println!("BAD EXPR{:#?}\n", expr);
            unimplemented!()
        }
    }
}

pub fn segments_to_expr(s: Vec<segments::Segment>) -> Expr {
    if s.len() == 1 {
        match s[0].clone() {
            segments::Segment::UnMatched(tv) => return tokens_to_expr(tv.clone()),
            _ => {
                println!("ALONE{:#?}\n", s[0]);
                unimplemented!()
            }
        }
    }
    if s.len() == 2 {
        match (s[0].clone(), s[1].clone()) {
            (
                segments::Segment::UnMatched(um),
                segments::Segment::Clause(Clause {
                    head: tokens::Token::LeftP,
                    body,
                    ..
                }),
            ) => {
                if um.len() == 1 {
                    match um[0].clone() {
                        Token::Identifier(l) => {
                            let args = segments_to_call_args(body);
                            return Expr::Call(l, args);
                        }
                        _ => {
                            println!("{:#?}\n", s);
                            unimplemented!()
                        }
                    }
                }
            }
            _ => {
                println!("{:#?}\n", s);
                unimplemented!()
            }
        }
    }
    if s.len() == 3 {
        match (s[0].clone(), s[1].clone(), s[2].clone()) {
            (
                segments::Segment::Clause(Clause {
                    head: tokens::Token::LeftP,
                    body: sv,
                    ..
                }),
                segments::Segment::UnMatched(um),
                segments::Segment::Clause(Clause {
                    head: tokens::Token::LeftP,
                    body: sv2,
                    ..
                }),
            ) => {
                if um.len() == 1 {
                    match um[0] {
                        Token::Add => {
                            let left = segments_to_expr(sv.clone());
                            let right = segments_to_expr(sv2.to_vec());
                            return Expr::ADD(Box::new(left), Box::new(right));
                        }
                        _ => {
                            println!("{:#?}\n", s);
                            unimplemented!()
                        }
                    }
                } else {
                    println!("{:#?}\n", s);
                    unimplemented!()
                }
            }
            bad => {
                println!("{:#?}{:#?}\n", s, bad);
                unimplemented!()
            }
        }
    }
    // TODO handle longer logic expr like
    // a && b && c && d
    // not sure what () rules will be needed
    println!("{:#?}\n", s);
    unimplemented!()
}

fn tokens_to_expr(tv: Vec<Token>) -> Expr {
    match tv.len() {
        1 => match tv[0].clone() {
            Token::Identifier(l) => return Expr::Identifier(l),
            _ => {
                println!("1{:#?}\n", tv);
                unimplemented!()
            }
        },
        3 => match (tv[0].clone(), tv[1].clone(), tv[2].clone()) {
            (Token::Identifier(l), Token::Add, Token::Identifier(r)) => {
                return Expr::ADD(Box::new(Expr::Identifier(l)), Box::new(Expr::Identifier(r)))
            }
            _ => {
                println!("3{:#?}\n", tv);
                unimplemented!()
            }
        },
        _ => {
            println!("{:#?}\n", tv);
            unimplemented!()
        }
    }
}

fn segments_to_call_args(seg: Vec<segments::Segment>) -> Vec<Expr> {
    if seg.len() == 0 {
        return Vec::new();
    }
    if seg.len() == 1 {
        match seg[0].clone() {
            segments::Segment::UnMatched(tv) => {
                if tv.len() == 3 {
                    match (tv[0].clone(), tv[1].clone(), tv[2].clone()) {
                        (
                            tokens::Token::Identifier(l),
                            tokens::Token::ArgTerm,
                            tokens::Token::Identifier(r),
                        ) => {
                            let mut res = Vec::new();
                            res.push(string_token_to_expr(l));
                            res.push(string_token_to_expr(r));
                            return res;
                        }
                        _ => {
                            println!("args{:#?}\n", seg);
                            unimplemented!()
                        }
                    }
                }
                println!("args len{:#?}\n", seg);
                unimplemented!()
            }
            _ => {
                println!("args{:#?}\n", seg);
                unimplemented!()
            }
        }
    }
    println!("args{:#?}\n", seg);
    unimplemented!()
}

pub fn string_token_to_expr(chars: Vec<char>) -> Expr {
    let text = String::from_iter(chars.iter());
    if let Ok(n) = text.parse::<usize>() {
        return Expr::Constant(function::Data::Number(n));
    } else {
        return Expr::Identifier(chars);
    }
}
