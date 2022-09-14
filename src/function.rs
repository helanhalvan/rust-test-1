use std::{borrow::Borrow, collections::HashMap};

use crate::{expr, logic_expr, program};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
    Number(usize),
    Boolean(bool),
    Emptylist,
    List(Box<Data>, Box<Data>),
}

pub type Program = HashMap<Vec<char>, program::Fun>;
pub type ProgramState = HashMap<Vec<char>, Data>;

pub fn call(code: Program, function: Vec<char>, args: Vec<Data>) -> Data {
    match code.get(&function) {
        Some(program::Fun { binds, .. }) => {
            let (state0, filters) = bind_args(binds.clone(), args.clone());
            let body = get_callpath(code.clone(), state0.clone(), filters.clone());
            let res = expr::eval(code, state0.clone(), body.clone());
            //println!("B{:#?}{:#?}{:#?}{:#?}\n", body, state0, filters, res);
            return res;
        }
        None => {
            println!("No Function:{:#?}{:#?}{:#?}\n", code, function, args);
            unimplemented!();
        }
    }
}

fn get_callpath(c: Program, p: ProgramState, fv: Vec<program::Filter>) -> expr::Expr {
    match fv.split_first() {
        Some((program::Filter { head, code }, t)) => {
            let passed_filter = logic_expr::eval(c.clone(), p.clone(), head.clone());
            if passed_filter {
                return code.clone();
            } else {
                return get_callpath(c, p, t.to_vec());
            }
        }
        None => {
            println!("No Pattern:{:#?}{:#?}\n", p, fv);
            unimplemented!();
        }
    }
}

fn bind_args(binds: Vec<program::Bind>, args: Vec<Data>) -> (ProgramState, Vec<program::Filter>) {
    match binds.split_first() {
        Some((program::Bind { pattern, filters }, t)) => {
            match try_bind(pattern.to_vec(), args.clone()) {
                Some(state) => {
                    return (state, filters.clone());
                }
                _ => bind_args(t.to_vec(), args),
            }
        }
        None => {
            println!("No Pattern:{:#?}{:#?}\n", binds, args);
            unimplemented!();
        }
    }
}

fn try_bind(pattern: Vec<program::ArgBind>, args: Vec<Data>) -> Option<ProgramState> {
    return try_bind_int(HashMap::new(), pattern, args);
}

fn try_bind_int(
    mut state: ProgramState,
    pattern: Vec<program::ArgBind>,
    args: Vec<Data>,
) -> Option<ProgramState> {
    if pattern.len() != args.len() {
        println!("bind len missmatch{:#?}{:#?}\n", pattern, args);
        unimplemented!();
    }
    if pattern.len() == 0 {
        return Some(state);
    }
    match (pattern.split_first(), args.split_first()) {
        (Some((ph, pt)), Some((ah, at))) => {
            let ph1 = ph.clone();
            let ah1 = ah.clone();
            if let Some(state1) = try_bind_single(state, ph1, ah1) {
                let pt1 = pt.to_vec();
                let at1 = at.to_vec();
                return try_bind_int(state1, pt1, at1);
            } else {
                return None;
            }
        }
        a => {
            println!("try_bind{:#?}{:#?}{:#?}\n", pattern, args, a);
            return None;
        }
    }
}

fn try_bind_single(
    mut state: ProgramState,
    pattern: program::ArgBind,
    arg: Data,
) -> Option<ProgramState> {
    match (pattern.clone(), arg.clone()) {
        (program::ArgBind::ConstPattern(c1), c2) => {
            if c1 == c2 {
                return Some(state);
            } else {
                return None;
            }
        }
        (program::ArgBind::Identifier(ph), _) => match state.get(&ph) {
            None => {
                state.insert(ph.to_vec(), arg);
                return Some(state);
            }
            _ => {
                println!("pattern matching{:#?}{:#?}{:#?}\n", state, pattern, arg);
                unimplemented!();
            }
        },
        (program::ArgBind::ListPattern { head: ah, tail: at }, Data::List(dh, dt)) => {
            let dh1 = *dh.clone();
            let ah1 = *ah.clone();
            if let Some(state1) = try_bind_single(state, ah1, dh1) {
                let dt1 = *dt.clone();
                let at1 = *at.clone();
                if let Some(state2) = try_bind_single(state1, at1, dt1) {
                    return Some(state2);
                } else {
                    return None;
                }
            } else {
                println!("try_bind{:#?}{:#?}\n", pattern, arg);
                return None;
            }
        }
        a => {
            println!("failed to bind single{:#?}\n", a);
            return None;
        }
    }
}
