pub mod segments;
pub mod tokens;

use std::{fs, println};

use segments::{Clause, Segment}; //, vec

fn main() {
    let c = fs::read_to_string("./program.txt").expect("Cant read file");
    let c2 = clean(c);
    let tokens = tokens::string_to(c2.clone());
    // Groups of tokes for expressing a single case within the code base
    let clauses = segments::matched_terminators(tokens.clone());
    let clauses2 = segments::prune_clauses(clauses.clone());
    let funs = to_funs(clauses2.clone());
    println!("{:#?}\n", (c2, funs))
}

fn clean(s: String) -> String {
    return s.chars().filter(|c| !c.is_whitespace()).collect();
}

fn to_funs(clauses: Vec<segments::Segment>) -> Vec<Fun> {
    return funs_int(clauses, Vec::new());
}
fn funs_int(clauses: Vec<segments::Segment>, mut done: Vec<Fun>) -> Vec<Fun> {
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
                done.insert(0, cfun);
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
            println!("{:#?}\n", f);
            unimplemented!()
        }
    }
}

fn bind_segment_to_bindpattern(s: Vec<Segment>) -> Vec<ArgBind> {
    if s.len() == 0 {
        return Vec::new();
    }
    match s[0].clone() {
        segments::Segment::UnMatched(i) => return bind_tokens_to_bindpattern(i),
        // todo handle [], [H|T], etc
        seg => {
            println!("{:#?}\n", seg);
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
    if let Some(c) = try_token_to_constaint(t.clone()) {
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

fn try_token_to_constaint(t: tokens::Token) -> Option<ArgBind> {
    match t {
        _ => {
            println!("{:#?}\n", t);
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
                    head: Vec::new(),
                    code: codebody,
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
            let cfilter = Filter {
                head: filterbody,
                code: codebody,
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
                head: Vec::new(),
                code: codebody,
            };
            let (mut otherfilters, finalrest) = segments_to_filters(rest);
            otherfilters.insert(0, cfilter);
            return (otherfilters, finalrest);
        }
        _ => return (Vec::new(), s),
    }
}

#[derive(Debug, Clone)]
pub struct Fun {
    name: Vec<char>,
    binds: Vec<Bind>,
}

#[derive(Debug, Clone)]
struct Bind {
    pattern: Vec<ArgBind>,
    filters: Vec<Filter>,
}

#[derive(Debug, Clone)]
enum ArgBind {
    Constant,
    Identifier(Vec<char>),
}

enum Constant {
    Emptylist,
}

#[derive(Debug, Clone)]
struct Filter {
    head: Vec<segments::Segment>,
    code: Vec<segments::Segment>,
}
