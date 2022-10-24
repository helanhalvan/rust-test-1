use crate::{
    function, program,
    segments::{self, Segment},
    tokens::{self, is_operator_token, Token},
};

use std::{borrow::Borrow, collections::VecDeque};

#[derive(Debug, Clone)]
pub enum CallLevel {
    // f(a,b,c) -> Call(f, [a,b,c])
    Call(function::FunctionName, Vec<CallLevel>),
    // a + b + c -> level(+, [a,b,c])
    OpLevel(tokens::Token, Vec<CallLevel>),
    // a = 5 a + 1 -> Assign(a, 5, a + 1)
    Assign {
        left: Box<CallLevel>,
        right: Box<CallLevel>,
        rest: Box<CallLevel>,
    },
    Identifier(Vec<char>),
    // [a,b,c] -> ListBuild([a,b,c,[]])
    // [a|b] -> ListBuild([a,b])
    ListBuild(Vec<CallLevel>),
    Emptylist,
}

struct IntOpLevel {
    pub t: tokens::Token,
    pub v: Vec<CallLevel>,
}

pub fn segments_to_call_level(
    mut s: Vec<segments::Segment>,
    context: program::Functions,
) -> CallLevel {
    match s.len() {
        1 => segment_to_call_level(s[0].clone()),
        i if i > 1 => match (s[0].clone(), s[1].clone()) {
            (
                segments::Segment::UnMatched(mut v),
                segments::Segment::Clause {
                    head: tokens::Token::LeftP,
                    body,
                    ..
                },
            ) => {
                match v.pop() {
                    // 1 + f(a,b) etc
                    Some(tokens::Token::Identifier(fname)) => {
                        let fun = make_fun(fname.clone(), body.clone(), context.clone());
                        match v.pop() {
                            Some(optoken) if is_operator_token(optoken.clone()) => {
                                let IntOpLevel {
                                    t: optoken,
                                    v: mut rest,
                                } = build_level_int(v, optoken, vec![]);
                                rest.push(fun);
                                return CallLevel::OpLevel(optoken, rest);
                            }
                            None => {
                                return fun;
                            }
                            Some(tokens::Token::Assign) => {
                                let left = tokens_to_call_level(v);
                                let rest =
                                    segments_to_call_level(s[2..s.len()].to_vec(), context.clone());
                                return CallLevel::Assign {
                                    left: Box::new(left),
                                    right: Box::new(fun),
                                    rest: Box::new(rest),
                                };
                            }
                            Some(Token::Qualify) => match v.pop() {
                                Some(Token::Identifier(id)) => {
                                    match (id.get(0), id.get(1), id.get(2), id.get(3), id.get(4)) {
                                        (Some('r'), Some('u'), Some('s'), Some('t'), None) => {
                                            return make_rust_fun(fname.clone(), body);
                                        }
                                        token => {
                                            println!(
                                                "SEG LEN CONFUSION{:#?}{:#?}{:#?}\n",
                                                s,
                                                s.len(),
                                                token
                                            );
                                            unimplemented!()
                                        }
                                    }
                                }
                                token => {
                                    println!(
                                        "SEG LEN CONFUSION{:#?}{:#?}{:#?}\n",
                                        s,
                                        s.len(),
                                        token
                                    );
                                    unimplemented!()
                                }
                            },
                            token => {
                                println!("SEG LEN CONFUSION{:#?}{:#?}{:#?}\n", s, s.len(), token);
                                unimplemented!()
                            }
                        }
                    }
                    token => {
                        println!("SEG LEN CONFUSION{:#?}{:#?}{:#?}\n", s, s.len(), token);
                        unimplemented!()
                    }
                }
            }
            (
                segments::Segment::Clause {
                    head: tokens::Token::LeftP,
                    body,
                    ..
                },
                segments::Segment::UnMatched(mut tv),
            ) => match tv.get(0) {
                Some(optoken) if is_operator_token(optoken.clone()) => {
                    let firstsub = segments_to_call_level(body, context);
                    return int_oplevel_to_oplevel(build_level_int(
                        tv[1..tv.len()].to_vec(),
                        optoken.clone(),
                        vec![firstsub],
                    ));
                }
                token => {
                    println!("SEG LEN CONFUSION{:#?}{:#?}{:#?}\n", s, s.len(), token);
                    unimplemented!()
                }
            },
            a => {
                println!("SEG LEN CONFUSION{:#?}{:#?}\n", s, a);
                unimplemented!()
            }
        },
        _ => {
            println!("SEG LEN CONFUSION{:#?}{:#?}\n", s, s.len());
            unimplemented!()
        }
    }
}

fn segment_to_call_level(seg: segments::Segment) -> CallLevel {
    match seg.clone() {
        segments::Segment::UnMatched(tv) => return tokens_to_call_level(tv.clone()),
        segments::Segment::Clause {
            head: Token::LeftB,
            body,
            ..
        } => listbuild(body),
        _ => {
            println!("ALONE3{:#?}\n", seg);
            unimplemented!()
        }
    }
}

fn tokens_to_call_level(tv: Vec<Token>) -> CallLevel {
    match tv.len() {
        1 => match tv[0].clone() {
            Token::Identifier(l) => return CallLevel::Identifier(l),
            _ => {
                println!("1{:#?}\n", tv);
                unimplemented!()
            }
        },
        I if I > 2 => match (tv[0].clone(), tv[1].clone()) {
            (Token::Identifier(l), token) if is_operator_token(token.clone()) => {
                let mut first = Vec::new();
                first.push(CallLevel::Identifier(l));
                return int_oplevel_to_oplevel(build_level_int(
                    tv[2..tv.len()].to_vec(),
                    token,
                    first,
                ));
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

// 1 + 2 + 3 + 4 ... etc
fn build_level_int(tv: Vec<Token>, leveltoken: Token, mut done: Vec<CallLevel>) -> IntOpLevel {
    assert!(
        tokens::is_operator_token(leveltoken.clone()),
        "{:#?}{:#?}{:#?}",
        tv,
        leveltoken,
        done
    );
    match tv.len() {
        1 => match tv[0].clone() {
            Token::Identifier(l) => {
                done.push(CallLevel::Identifier(l));
                return IntOpLevel {
                    t: leveltoken,
                    v: done,
                };
            }
            _ => {
                println!("{:#?}\n", tv);
                unimplemented!()
            }
        },
        I if I > 2 => match (tv[0].clone(), tv[1].clone()) {
            (Token::Identifier(l), leveltoken) if is_operator_token(leveltoken.clone()) => {
                done.push(CallLevel::Identifier(l));
                build_level_int(tv[2..tv.len()].to_vec(), leveltoken, done)
            }
            _ => {
                println!("{:#?}\n", tv);
                unimplemented!()
            }
        },
        _ => {
            println!("{:#?}\n", tv);
            unimplemented!()
        }
    }
}

fn listbuild(body: Vec<Segment>) -> CallLevel {
    match (body.get(0), body.get(1), body.get(2)) {
        (Some(segments::Segment::UnMatched(tv)), None, None) => {
            return tokens_to_list(tv.clone());
        }
        (None, None, None) => {
            return CallLevel::ListBuild(Vec::new());
        }
        (a, b, c) => {
            println!("LISTBUILD{:#?}{:#?}{:#?}{:#?}\n", body, a, b, c);
            unimplemented!()
        }
    }
}

// gets tokens *between* the brackets as arg
fn tokens_to_list(mut tv: Vec<Token>) -> CallLevel {
    if tv.len() == 3 {
        let last = tv.pop().unwrap();
        let middle = tv.pop().unwrap();
        let first = tv.pop().unwrap();
        match (first, middle, last) {
            (Token::Identifier(f), Token::ArgTerm, Token::Identifier(s)) => {
                let first = CallLevel::Identifier(f);
                let second = CallLevel::Identifier(s);
                let mut items = Vec::new();
                items.push(first);
                items.push(second);
                items.push(CallLevel::Emptylist);
                let list = CallLevel::ListBuild(items);
                return list;
            }
            (Token::Identifier(f), Token::Pipe, Token::Identifier(s)) => {
                let first = CallLevel::Identifier(f);
                let mut items = Vec::new();
                let second = CallLevel::Identifier(s);
                items.push(first);
                items.push(second);
                let list = CallLevel::ListBuild(items);
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

fn make_fun(
    fname: Vec<char>,
    args: Vec<segments::Segment>,
    context: program::Functions,
) -> CallLevel {
    let args = segments_to_call_args(args);
    if context.contains_key(&fname) {
        return CallLevel::Call(function::FunctionName::Static(fname), args);
    } else {
        return CallLevel::Call(function::FunctionName::Dynamic(fname), args);
    }
}

fn make_rust_fun(fname: Vec<char>, args: Vec<segments::Segment>) -> CallLevel {
    let args = segments_to_call_args(args);
    return CallLevel::Call(function::FunctionName::Rust(fname), args);
}

fn segments_to_call_args(mut seg: Vec<segments::Segment>) -> Vec<CallLevel> {
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
                res.push(segment_to_call_level(a));
                return res;
            }
        }
    }
    if seg.len() == 2 {
        match (seg[0].clone(), seg[1].clone()) {
            (segments::Segment::UnMatched(tv), tailseg) => {
                if let Some((tokens::Token::ArgTerm, tail)) = tv.split_last() {
                    let mut res = tokens_to_call_args(tail.to_vec());
                    res.push(segment_to_call_level(tailseg));
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
                        let tailarg = segment_to_call_level(tailseg);
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

fn tokens_to_call_args(mut tv: Vec<Token>) -> Vec<CallLevel> {
    if tv.len() == 1 {
        match tv[0].clone() {
            tokens::Token::Identifier(l) => {
                let mut res = Vec::new();
                res.push(CallLevel::Identifier(l));
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
                res.push(CallLevel::Identifier(l));
                return res;
            }
            // cases like add(1+1) where call-args
            // are not single tokens
            // TODO handle add(a, 1+1, a)
            _a => {
                let mut res = Vec::new();
                res.push(tokens_to_call_level(fulltv));
                return res;
            }
        }
    }
    println!("args len{:#?}\n", tv);
    unimplemented!()
}

fn int_oplevel_to_oplevel(IntOpLevel { t, v }: IntOpLevel) -> CallLevel {
    return CallLevel::OpLevel(t, v);
}
