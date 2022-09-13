use std::{borrow::Borrow, collections::HashMap};

use crate::{expr, logic_expr, program};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
    Number(usize),
    Boolean(bool),
}

pub type Program = HashMap<Vec<char>, program::Fun>;
pub type ProgramState = HashMap<Vec<char>, Data>;

pub fn call(code: Program, function: Vec<char>, args: Vec<Data>) -> Data {
    match code.get(&function) {
        Some(program::Fun { binds, .. }) => {
            let (state0, filters) = bind_args(binds.clone(), args.clone());
            println!("BOUND{:#?}{:#?}\n", state0, filters);
            let body = get_callpath(code.clone(), state0.clone(), filters.clone());
            let res = expr::eval(code, state0.clone(), body.clone());
            println!("B{:#?}{:#?}{:#?}{:#?}\n", body, state0, filters, res);
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
) -> Option<HashMap<Vec<char>, Data>> {
    if pattern.len() != args.len() {
        println!("bind len missmatch{:#?}{:#?}\n", pattern, args);
        unimplemented!();
    }
    if pattern.len() == 0 {
        return Some(state);
    }
    match (pattern.split_first(), args.split_first()) {
        (Some((program::ArgBind::Identifier(ph), pt)), Some((ah, at))) => match state.get(ph) {
            None => {
                state.insert(ph.to_vec(), ah.clone());
                return try_bind_int(state, pt.to_vec(), at.to_vec());
            }
            _ => {
                println!("pattern matching{:#?}{:#?}{:#?}\n", state, pattern, args);
                unimplemented!();
            }
        },
        _ => {
            println!("try_bind{:#?}{:#?}\n", pattern, args);
            return None;
        }
    }
}
