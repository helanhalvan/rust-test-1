use std::collections::HashMap;

use crate::{
    eval, expr, logic_expr,
    segments::{self, Clause, Segment},
    tokens,
};

#[derive(Debug, Clone)]
pub struct Fun {
    pub name: Vec<char>,
    pub binds: Vec<Bind>,
}

#[derive(Debug, Clone)]
pub struct Bind {
    pub pattern: Vec<ArgBind>,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Clone)]
pub enum ArgBind {
    Emptylist,
    ListPattern {
        head: Box<ArgBind>,
        tail: Box<ArgBind>,
    },
    ConstPattern(eval::Data),
    Identifier(Vec<char>),
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub head: logic_expr::LogicExpr,
    pub code: expr::Expr,
}

// if function names are duplicated
// the last version of a function will win
pub fn to_funs(clauses: Vec<segments::Segment>) -> HashMap<Vec<char>, Fun> {
    return funs_int(clauses, HashMap::new());
}
fn funs_int(
    clauses: Vec<segments::Segment>,
    mut done: HashMap<Vec<char>, Fun>,
) -> HashMap<Vec<char>, Fun> {
    if clauses.len() == 0 {
        return done;
    }
    match (clauses[0].clone(), clauses[1].clone()) {
        (
            segments::Segment::UnMatched(i), // [tokens::Token::Identifier(i)]
            segments::Segment::Clause(Clause {
                head: tokens::Token::FunStart,
                body: b,
                ..
            }),
        ) => match i[0].clone() {
            tokens::Token::Identifier(n) => {
                let rest = clauses[2..clauses.len()].to_vec();
                let cfun = Fun {
                    name: n.clone(),
                    binds: segments_to_binds(b),
                };
                done.insert(n.clone(), cfun);
                let rfun = funs_int(rest, done);
                return rfun;
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn segments_to_binds(binds: Vec<segments::Segment>) -> Vec<Bind> {
    if binds.len() == 0 {
        return Vec::new();
    }
    match binds[0].clone() {
        segments::Segment::Clause(Clause {
            head: tokens::Token::LeftP,
            body: bindbody,
            ..
        }) => {
            let rest = binds[1..binds.len()].to_vec();
            let (filters, rest) = segments_to_filters(rest);
            let mut obind = segments_to_binds(rest);
            let bindpattern = bind_segment_to_bindpattern(bindbody);
            let cbind = Bind {
                pattern: bindpattern,
                filters: filters,
            };
            obind.insert(0, cbind);
            return obind;
        }
        f => {
            println!("5{:#?}\n", f);
            unimplemented!()
        }
    }
}

fn bind_segment_to_bindpattern(s: Vec<Segment>) -> Vec<ArgBind> {
    if s.len() == 0 {
        return Vec::new();
    }
    if s.len() == 1 {
        return single_seg_to_bind(s[0].clone());
    }
    if s.len() == 2 {
        match s[1].clone() {
            segments::Segment::UnMatched(t) => match t.split_first() {
                Some((tokens::Token::ArgTerm, rest)) => {
                    let mut res = single_seg_to_bind(s[0].clone());
                    let mut more = bind_tokens_to_bindpattern(rest.to_vec());
                    res.append(&mut more);
                    return res;
                }
                A => {
                    println!("{:#?}\n", A);
                    unimplemented!()
                }
            },
            A => {
                println!("{:#?}\n", A);
                unimplemented!()
            }
        }
    }
    println!("BAD_BINDSEG{:#?}\n", s);
    unimplemented!()
}

fn single_seg_to_bind(seg: Segment) -> Vec<ArgBind> {
    match seg {
        // (A), (A, B), etc
        segments::Segment::UnMatched(i) => return bind_tokens_to_bindpattern(i),
        // []
        // [H|T], etc
        segments::Segment::Clause(Clause {
            head: tokens::Token::LeftB,
            body: i,
            ..
        }) => {
            if i.len() == 0 {
                let mut res = Vec::new();
                res.push(ArgBind::ConstPattern(eval::Data::Emptylist));
                return res;
            } else {
                match &i[0] {
                    segments::Segment::UnMatched(t) => {
                        if t.len() == 3 && t[1] == tokens::Token::Pipe {
                            let mut res = Vec::new();
                            let head = token_to_bind(t[0].clone());
                            let tail = token_to_bind(t[2].clone());
                            res.push(ArgBind::ListPattern {
                                head: Box::new(head),
                                tail: Box::new(tail),
                            });
                            return res;
                        }
                        println!("1{:#?}\n", t);
                        unimplemented!()
                    }
                    seg => {
                        println!("2{:#?}\n", seg);
                        unimplemented!()
                    }
                }
            }
        }
        seg => {
            println!("3{:#?}\n", seg);
            unimplemented!()
        }
    };
}

fn bind_tokens_to_bindpattern(s: Vec<tokens::Token>) -> Vec<ArgBind> {
    if s.len() == 0 {
        return Vec::new();
    }
    if s.len() == 1 {
        let bind = token_to_bind(s[0].clone());
        let mut args = Vec::new();
        args.insert(0, bind);
        return args;
    }
    // handing (A , ..)
    match s[1].clone() {
        tokens::Token::ArgTerm => {
            let bind = token_to_bind(s[0].clone());
            let rest = s[2..s.len()].to_vec();
            let mut args = bind_tokens_to_bindpattern(rest);
            args.insert(0, bind);
            return args;
        }
        _ => unimplemented!(),
    }
}

fn token_to_bind(t: tokens::Token) -> ArgBind {
    if let Some(c) = try_token_to_constant(t.clone()) {
        return c;
    } else if let Some(i) = try_token_to_identifier(t) {
        return i;
    } else {
        unimplemented!();
    };
}

fn try_token_to_identifier(t: tokens::Token) -> Option<ArgBind> {
    match t {
        tokens::Token::Identifier(c) => return Some(ArgBind::Identifier(c)),
        _ => return None,
    }
}

fn try_token_to_constant(t: tokens::Token) -> Option<ArgBind> {
    match t {
        // TODO handle [] etc
        _ => {
            return None;
        }
    }
}

fn segments_to_filters(s: Vec<segments::Segment>) -> (Vec<Filter>, Vec<Segment>) {
    if s.len() == 0 {
        return (Vec::new(), s);
    } else if s.len() == 1 {
        match s[0].clone() {
            segments::Segment::Clause(Clause {
                head: tokens::Token::CodeStart,
                body: codebody,
                ..
            }) => {
                let cfilter = Filter {
                    head: logic_expr::LogicExpr::True,
                    code: expr::segments_to_expr(codebody),
                };
                let mut allfilters = Vec::new();
                allfilters.insert(0, cfilter);
                return (allfilters, Vec::new());
            }
            _ => unimplemented!(),
        }
    }
    match (s[0].clone(), s[1].clone()) {
        (
            segments::Segment::Clause(Clause {
                head: tokens::Token::LeftW,
                body: filterbody,
                ..
            }),
            segments::Segment::Clause(Clause {
                head: tokens::Token::CodeStart,
                body: codebody,
                ..
            }),
        ) => {
            let rest = s[2..s.len()].to_vec();
            let (mut otherfilters, finalrest) = segments_to_filters(rest);
            let filter = logic_expr::segments_to_logical_expr(filterbody);
            let cfilter = Filter {
                head: filter,
                code: expr::segments_to_expr(codebody),
            };
            otherfilters.insert(0, cfilter);
            return (otherfilters, finalrest);
        }
        (
            segments::Segment::Clause(Clause {
                head: tokens::Token::CodeStart,
                body: codebody,
                ..
            }),
            _,
        ) => {
            let rest = s[1..s.len()].to_vec();
            let cfilter = Filter {
                head: logic_expr::LogicExpr::True,
                code: expr::segments_to_expr(codebody),
            };
            let (mut otherfilters, finalrest) = segments_to_filters(rest);
            otherfilters.insert(0, cfilter);
            return (otherfilters, finalrest);
        }
        _ => return (Vec::new(), s),
    }
}
