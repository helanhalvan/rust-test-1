use std::collections::{HashMap, HashSet};

use crate::{
    call_levels, eval, expr, function, logic_expr, pattern_match,
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

#[derive(Debug, Clone)]
pub struct FunFun {
    pub name: Vec<char>,
    pub binds: Vec<FunBind>,
}

#[derive(Debug, Clone)]
pub struct FunBind {
    pub pattern: Vec<pattern_match::ArgBind>,
    pub filters: Vec<FunFilter>,
}

#[derive(Debug, Clone)]
pub struct FunFilter {
    pub head: Vec<segments::Segment>,
    pub code: Vec<segments::Segment>,
}

pub type Functions = HashMap<Vec<char>, FunFun>;

// if function names are duplicated
// the last version of a function will win
pub fn to_funs(clauses: Vec<segments::Segment>) -> Functions {
    return funs_int(clauses, HashMap::new());
}

pub fn to_program(funs: Functions) -> eval::Program {
    let staticnames = funs.clone();
    let res = funs
        .into_iter()
        .map(|(n, x)| {
            (
                function::FunctionName::Static(n.to_vec()),
                funfun_to_fun(x.clone(), staticnames.clone()),
            )
        })
        .collect();
    return res;
}

fn funfun_to_fun(f: FunFun, context: Functions) -> Fun {
    return Fun {
        binds: f
            .binds
            .into_iter()
            .map(|b| funbind_to_bind(b, context.clone()))
            .collect(),
        name: f.name,
    };
}

fn funbind_to_bind(b: FunBind, context: Functions) -> Bind {
    return Bind {
        pattern: b.pattern,
        filters: b
            .filters
            .into_iter()
            .map(|x| funfilter_to_filter(x, context.clone()))
            .collect(),
    };
}

fn funfilter_to_filter(f: FunFilter, context: Functions) -> Filter {
    let h = if f.head.is_empty() {
        logic_expr::LogicExpr::True // no filter -> always true filter
    } else {
        let h = call_levels::segments_to_call_level(f.head, context.clone());
        logic_expr::call_levels_to_logic_expr(h)
    };
    let c = call_levels::segments_to_call_level(f.code, context);
    return Filter {
        head: h,
        code: expr::call_levels_to_expr(c),
    };
}

fn funs_int(clauses: Vec<segments::Segment>, mut done: Functions) -> Functions {
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
                let cfun = FunFun {
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

fn segments_to_binds(binds: Vec<segments::Segment>) -> Vec<FunBind> {
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
            let cbind = FunBind {
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

fn segments_to_filters(s: Vec<segments::Segment>) -> (Vec<FunFilter>, Vec<Segment>) {
    if s.len() == 0 {
        return (Vec::new(), s);
    } else if s.len() == 1 {
        match s[0].clone() {
            segments::Segment::Clause {
                head: tokens::Token::CodeStart,
                body: codebody,
                ..
            } => {
                let cfilter = FunFilter {
                    head: Vec::new(),
                    code: codebody,
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
            let cfilter = FunFilter {
                head: filterbody,
                code: codebody,
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
            let cfilter = FunFilter {
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
