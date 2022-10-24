use std::{borrow::Borrow, collections::HashSet};

use crate::{
    eval::{self, Program},
    expr::Expr,
    logic_expr::LogicExpr,
    program::{self, Fun},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionName {
    Static(Vec<char>),
    Dynamic(Vec<char>),
    Rust(Vec<char>),
}

pub fn call(
    code: Program,
    state: eval::ProgramState,
    function: FunctionName,
    args: Vec<eval::Data>,
) -> eval::Data {
    match function.clone() {
        FunctionName::Static(key) => match code.get(&FunctionName::Static(key)) {
            None => {
                println!(
                    "No Pattern:{:#?}{:#?}{:#?}{:#?}\n",
                    code, state, function, args
                );
                unimplemented!();
            }
            Some(f) => eval::call(code.clone(), f.clone(), args),
        },
        FunctionName::Dynamic(key) => match state.get(&key) {
            Some(eval::Data::FunctionPointer(f)) => call(code, state.clone(), f.clone(), args),
            bad => {
                println!(
                    "No Pattern:{:#?}{:#?}{:#?}{:#?}{:#?}\n",
                    code, state, function, args, bad
                );
                unimplemented!();
            }
        },
        FunctionName::Rust(key) => {
            return rust_call(key, args);
        }
    }
}

fn rust_call(name: Vec<char>, args: Vec<eval::Data>) -> eval::Data {
    match (name.get(0), name.get(1), name.get(2), name.get(3)) {
        (Some('l'), Some('o'), Some('g'), None) => {
            println!("LANGLOG{:#?}\n", args);
            return eval::Data::Emptylist;
        }
        bad => {
            println!("No Pattern:{:#?}{:#?}{:#?}\n", name, args, bad);
            unimplemented!();
        }
    }
}
