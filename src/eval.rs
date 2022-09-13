use std::{borrow::Borrow, collections::HashMap};

use crate::{expr, logic_expr, program};

#[derive(Debug, Clone)]
pub enum Data {
    Number(usize),
}

type Program = HashMap<Vec<char>, program::Fun>;
pub type ProgramState = HashMap<Vec<char>, Data>;

pub fn call(code: Program, function: Vec<char>, args: Vec<Data>) -> Data {
    match code.get(&function) {
        Some(program::Fun { binds, .. }) => {
            let (state0, filters) = bind_args(binds.clone(), args.clone());
            let body = get_callpath(state0.clone(), filters.clone());
            let (state, res) = expr::eval(state0, body);
            println!("B{:#?}{:#?}{:#?}\n", body, state0, filters);
            unimplemented!()
        }
        None => {
            println!("No Function:{:#?}{:#?}{:#?}\n", code, function, args);
            unimplemented!();
        }
    }
}

fn get_callpath(p: ProgramState, fv: Vec<program::Filter>) -> expr::Expr {
    match fv.split_first() {
        Some((program::Filter { head, code }, t)) => {
            let passed_filter = logic_expr::eval(p.clone(), head.clone());
            if passed_filter {
                return code.clone();
            } else {
                return get_callpath(p, t.to_vec());
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

fn try_bind(pattern: Vec<program::ArgBind>, args: Vec<Data>) -> Option<HashMap<Vec<char>, Data>> {
    if pattern.len() != args.len() {
        println!("bind len missmatch{:#?}{:#?}\n", pattern, args);
        unimplemented!();
    }
    if pattern.len() == 0 {
        return Some(HashMap::new());
    }
    println!("try_bind{:#?}{:#?}\n", pattern, args);
    return None;
}
