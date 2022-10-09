use std::collections::HashMap;

use crate::{
    eval, expr, logic_expr, pattern_match,
    segments::{self, Segment},
    tokens,
};

#[derive(Debug, Clone)]
pub struct Fun {
    pub name: Vec<char>,
    pub binds: Vec<Bind>,
}

#[derive(Debug, Clone)]
pub struct Bind {
    pub pattern: Vec<pattern_match::ArgBind>,
    pub filters: Vec<Filter>,
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
            segments::Segment::Clause {
                head: tokens::Token::FunStart,
                body: b,
                ..
            },
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
        segments::Segment::Clause {
            head: tokens::Token::LeftP,
            body: bindbody,
            ..
        } => {
            let rest = binds[1..binds.len()].to_vec();
            let (filters, rest) = segments_to_filters(rest);
            let mut obind = segments_to_binds(rest);
            let bindpattern = pattern_match::bind_segment_to_bindpattern(bindbody);
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

fn segments_to_filters(s: Vec<segments::Segment>) -> (Vec<Filter>, Vec<Segment>) {
    if s.len() == 0 {
        return (Vec::new(), s);
    } else if s.len() == 1 {
        match s[0].clone() {
            segments::Segment::Clause {
                head: tokens::Token::CodeStart,
                body: codebody,
                ..
            } => {
                let cfilter = Filter {
                    head: logic_expr::LogicExpr::True,
                    code: expr::segments_to_expr(codebody),
                };
                let mut allfilters = Vec::new();
                allfilters.insert(0, cfilter);
                return (allfilters, Vec::new());
            }
            _ => {
                println!("1{:#?}\n", s);
                unimplemented!()
            }
        }
    }
    match (s[0].clone(), s[1].clone()) {
        (
            segments::Segment::Clause {
                head: tokens::Token::LeftW,
                body: filterbody,
                ..
            },
            segments::Segment::Clause {
                head: tokens::Token::CodeStart,
                body: codebody,
                ..
            },
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
            segments::Segment::Clause {
                head: tokens::Token::CodeStart,
                body: codebody,
                ..
            },
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
