use std::iter::FromIterator;

use crate::eval::{self, Data, ProgramState};
use crate::logic_expr::{self, LogicExpr};
use crate::numeric_expr::{NumericData, NumericExpr};
use crate::segments::{self, Segment};
use crate::tokens::{self, Token};
use crate::{call_levels, function, numeric_expr, pattern_match, program};

//Un-typed Expressions
#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(Vec<char>),
    Constant(eval::Data),
    Call(function::FunctionName, Vec<Expr>),
    ListBuild(Box<Expr>, Box<Expr>), // Do we need this to be a [|] cell?
    NumericExpr(numeric_expr::NumericExpr),
    LogicExpr(LogicExpr),
    Assign {
        pattern: Box<pattern_match::ArgBind>,
        arg: Box<Expr>,
        rest: Box<Expr>,
    },
}

pub fn eval(c: eval::Program, p: eval::ProgramState, expr: Expr) -> eval::Data {
    match expr.clone() {
        Expr::Call(f, args) => eval_and_call(c, f, args, p),
        Expr::Constant(c) => {
            return c;
        }
        Expr::Identifier(l) => var_lookup(l, p),
        Expr::ListBuild(h, t) => {
            let h1 = eval(c.clone(), p.clone(), *h);
            let t1 = eval(c.clone(), p.clone(), *t);
            return eval::Data::List(Box::new(h1), Box::new(t1));
        }
        Expr::NumericExpr(nexpr) => {
            println!("NEXPR{:#?}\n", nexpr);
            return numeric_expr::eval(c, p, nexpr);
        }
        Expr::LogicExpr(lexpr) => return Data::Boolean(logic_expr::eval(c, p, lexpr)),
        Expr::Assign {
            pattern: bind,
            arg,
            rest,
        } => {
            let vals = vec![eval(c.clone(), p.clone(), *arg)];
            let binds = vec![*bind];
            if let Some(p1) = eval::try_bind_with_state(p.clone(), binds, vals) {
                return eval(c, p1, *rest);
            } else {
                println!("Bind Failed{:#?}{:#?}\n", expr, p);
                unimplemented!()
            }
        }
    }
}

pub fn var_lookup(name: Vec<char>, p: eval::ProgramState) -> eval::Data {
    match p.get(&name) {
        Some(v) => return v.clone(),
        _ => {
            println!("UNBOUND VAR{:#?}\n{:#?}\n", p, name);
            unimplemented!()
        }
    }
}

pub fn dynamic_eval_and_call(
    c: eval::Program,
    name: Vec<char>,
    args: Vec<Expr>,
    p: eval::ProgramState,
) -> eval::Data {
    match p.get(&name) {
        Some(eval::Data::FunctionPointer(fname)) => eval_and_call(c, fname.clone(), args, p),
        _ => {
            println!("NO FNAME{:#?}\n", name);
            unimplemented!()
        }
    }
}

pub fn eval_and_call(
    c: eval::Program,
    f: function::FunctionName,
    args: Vec<Expr>,
    p: eval::ProgramState,
) -> eval::Data {
    let args1 = args
        .into_iter()
        .map(|arg| eval(c.clone(), p.clone(), arg))
        .collect();
    return function::call(c, p, f, args1);
}

pub fn call_levels_to_expr(level: call_levels::CallLevel) -> Expr {
    match level.clone() {
        call_levels::CallLevel::OpLevel(token, sublevels) => match token {
            Token::Eq => {
                let subs = sublevels
                    .into_iter()
                    .map(|x| call_levels_to_expr(x))
                    .collect();
                return Expr::LogicExpr(LogicExpr::EQ(subs));
            }
            Token::Add | Token::SUB | Token::MUL => {
                return Expr::NumericExpr(numeric_expr::call_levels_to_num_expr(level));
            }
            t => {
                println!("call_levels_to_expr {:#?}{:#?}\n", level, t);
                unimplemented!()
            }
        },
        call_levels::CallLevel::Identifier(v) => return string_token_to_expr(v),
        call_levels::CallLevel::Call(fname, sublevels) => {
            let subs = sublevels
                .into_iter()
                .map(|x| call_levels_to_expr(x))
                .collect();
            return Expr::Call(fname, subs);
        }
        call_levels::CallLevel::Assign { left, right, rest } => {
            let left1 = pattern_match::call_level_to_argbind(*left);
            let right1 = call_levels_to_expr(*right);
            let rest1 = call_levels_to_expr(*rest);
            return Expr::Assign {
                pattern: Box::new(left1),
                arg: Box::new(right1),
                rest: Box::new(rest1),
            };
        }
        call_levels::CallLevel::ListBuild(sublevels) => {
            let subs: Vec<_> = sublevels
                .into_iter()
                .map(|x| call_levels_to_expr(x))
                .collect();
            return expr_vec_to_list(&subs);
        }
        _ => {
            println!("call_levels_to_expr {:#?}\n", level);
            unimplemented!()
        }
    }
}

fn expr_vec_to_list(v: &[Expr]) -> Expr {
    println!("listbuild {:#?}\n", v);
    match (v.get(0), v.get(1)) {
        (Some(x), Some(_)) => {
            println!("listbuild {:#?}\n", x);
            return Expr::ListBuild(
                Box::new(x.clone()),
                Box::new(expr_vec_to_list(&v[1..v.len()])),
            );
        }
        (Some(x), None) => {
            return x.clone();
        }
        (None, _) => {
            println!("call_levels_to_expr {:#?}\n", v);
            unimplemented!()
        }
    }
}

pub fn string_token_to_expr(chars: Vec<char>) -> Expr {
    let text = String::from_iter(chars.iter());
    if let Ok(n) = text.parse::<i64>() {
        return Expr::Constant(eval::Data::Number(NumericData::Int(n)));
    } else if let Ok(n) = text.parse::<bool>() {
        return Expr::Constant(eval::Data::Boolean(n));
    } else {
        return Expr::Identifier(chars);
    }
}
