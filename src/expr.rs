use std::iter::FromIterator;

use crate::function::{self, Data, ProgramState};
use crate::program;
use crate::segments::{self, Clause, Segment};
use crate::tokens::{self, Token};

//TODO we will probably need a lot of expr subtypes
#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(Vec<char>),
    Constant(function::Data),
    ADD(Box<Expr>, Box<Expr>),
    Call(Vec<char>, Vec<Expr>),
    ListBuild(Box<Expr>, Box<Expr>),
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
        Expr::ListBuild(h, t) => {
            let h1 = eval(c.clone(), p.clone(), *h);
            let t1 = eval(c.clone(), p.clone(), *t);
            return function::Data::List(Box::new(h1), Box::new(t1));
        }
        _ => {
            println!("BAD EXPR{:#?}\n", expr);
            unimplemented!()
        }
    }
}

pub fn segments_to_expr(s: Vec<segments::Segment>) -> Expr {
    if s.len() == 1 {
        return segment_to_expr(s[0].clone());
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

fn segment_to_expr(seg: segments::Segment) -> Expr {
    match seg.clone() {
        segments::Segment::UnMatched(tv) => return tokens_to_expr(tv.clone()),
        segments::Segment::Clause(Clause {
            head: Token::LeftB,
            mut body,
            ..
        }) => match (body.get(0), body.get(1), body.get(2), body.get(3)) {
            (Some(segments::Segment::UnMatched(tv)), Some(tailseg), None, None) => {
                let mut tv1 = tv.clone();
                match tv1.pop() {
                    // [h|f(a)] handling
                    Some(tokens::Token::Identifier(l)) => {
                        if let (
                            segments::Segment::Clause(Clause {
                                head: tokens::Token::LeftP,
                                body,
                                ..
                            }),
                            Some(tokens::Token::Pipe),
                        ) = (tailseg, tv1.pop())
                        {
                            let args = segments_to_call_args(body.to_vec());
                            let tail = Expr::Call(l, args);
                            let head = tokens_to_expr(tv1.to_vec());
                            return Expr::ListBuild(Box::new(head), Box::new(tail));
                        } else {
                            println!("ALONE1{:#?}{:#?}\n", tv, tailseg);
                            unimplemented!()
                        }
                    }
                    Some(tokens::Token::Pipe) => {
                        println!("BUILDING LIST{:#?}{:#?}\n", tv, tailseg);
                        unimplemented!()
                    }
                    _ => {
                        println!("ALONE1{:#?}{:#?}\n", tv, tailseg);
                        unimplemented!()
                    }
                }
            }
            (Some(segments::Segment::UnMatched(tv)), None, None, None) => {
                return tokens_to_list(tv.clone());
            }
            (None, None, None, None) => {
                return Expr::Constant(Data::Emptylist);
            }
            (a, b, c, d) => {
                println!("ALONE2{:#?}{:#?}{:#?}{:#?}{:#?}\n", seg, a, b, c, d);
                unimplemented!()
            }
        },
        _ => {
            println!("ALONE3{:#?}\n", seg);
            unimplemented!()
        }
    }
}

// gets tokens *between* the brackets as arg
fn tokens_to_list(mut tv: Vec<Token>) -> Expr {
    if tv.len() == 3 {
        let last = tv.pop().unwrap();
        let middle = tv.pop().unwrap();
        let first = tv.pop().unwrap();
        match (first, middle, last) {
            (Token::Identifier(f), Token::ArgTerm, Token::Identifier(s)) => {
                let first = string_token_to_expr(f);
                let second = string_token_to_expr(s);
                let empty = Expr::Constant(Data::Emptylist);
                let tail = Expr::ListBuild(Box::new(second), Box::new(empty));
                let list = Expr::ListBuild(Box::new(first), Box::new(tail));
                return list;
            }
            a => {
                println!("tokens_to_list{:#?}{:#?}\n", a, tv);
                unimplemented!()
            }
        }
    }
    println!("tokens_to_list{:#?}\n", tv);
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
                let l1 = string_token_to_expr(l);
                let r1 = string_token_to_expr(r);
                return Expr::ADD(Box::new(l1), Box::new(r1));
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

fn segments_to_call_args(mut seg: Vec<segments::Segment>) -> Vec<Expr> {
    if seg.len() == 0 {
        return Vec::new();
    }
    if seg.len() == 1 {
        match seg[0].clone() {
            // a, b, c etc
            segments::Segment::UnMatched(tv) => {
                return tokens_to_call_args(tv);
            }
            // [a, b], [a|b], etc
            a => {
                let mut res = Vec::new();
                res.push(segment_to_expr(a));
                return res;
            }
        }
    }
    if seg.len() == 2 {
        match (seg[0].clone(), seg[1].clone()) {
            (segments::Segment::UnMatched(tv), tailseg) => {
                if let Some((tokens::Token::ArgTerm, tail)) = tv.split_last() {
                    let mut res = tokens_to_call_args(tail.to_vec());
                    res.push(segment_to_expr(tailseg));
                    return res;
                } else {
                    println!("missing comma {:#?}\n", seg);
                    unimplemented!()
                }
            }
            a => {
                println!("2args{:#?}{:#?}\n", seg, a);
                unimplemented!()
            }
        }
    }
    // at least 3 arg function
    if seg.len() >= 3 {
        let last = seg.pop();
        let secondlast = seg.pop();
        match (secondlast, last) {
            (Some(segments::Segment::UnMatched(mut tv)), Some(tailseg)) => {
                let lasttoken = tv.pop();
                match (tv.split_first(), lasttoken) {
                    (Some((Token::ArgTerm, middletokens)), Some(Token::ArgTerm)) => {
                        let mut res = segments_to_call_args(seg);
                        let tailarg = segment_to_expr(tailseg);
                        let mut middleargs = tokens_to_call_args(middletokens.to_vec());
                        res.append(&mut middleargs);
                        res.push(tailarg);
                        return res;
                    }
                    a => {
                        println!("3 int args{:#?}{:#?}\n", seg, a);
                        unimplemented!()
                    }
                }
            }
            a => {
                println!("3args{:#?}{:#?}\n", seg, a);
                unimplemented!()
            }
        }
    }
    println!("args{:#?}\n", seg);
    unimplemented!()
}

fn tokens_to_call_args(mut tv: Vec<Token>) -> Vec<Expr> {
    if tv.len() == 1 {
        match tv[0].clone() {
            tokens::Token::Identifier(l) => {
                let mut res = Vec::new();
                res.push(string_token_to_expr(l));
                return res;
            }
            _ => {
                println!("args 1{:#?}\n", tv);
                unimplemented!()
            }
        }
    }
    if tv.len() >= 3 {
        let fulltv = tv.clone();
        let last = tv.pop();
        let secondlast = tv.pop();
        match (last, secondlast) {
            (Some(tokens::Token::Identifier(l)), Some(tokens::Token::ArgTerm)) => {
                let mut res = tokens_to_call_args(tv);
                res.push(string_token_to_expr(l));
                return res;
            }
            // cases like add(1+1) where call-args
            // are not single tokens
            // TODO handle add(a, 1+1, a)
            _a => {
                let mut res = Vec::new();
                res.push(tokens_to_expr(fulltv));
                return res;
            }
        }
    }
    println!("args len{:#?}\n", tv);
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
