use std::iter::FromIterator;

use crate::eval::{self, Data, ProgramState};
use crate::logic_expr::{self, LogicExpr};
use crate::numeric_expr::{NumericData, NumericExpr};
use crate::program::ArgBind;
use crate::segments::{self, Segment};
use crate::tokens::{self, Token};
use crate::{call_levels, function, numeric_expr, program};

//Un-typed Expressions
#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(Vec<char>),
    Constant(eval::Data),
    Call(Vec<char>, Vec<Expr>),
    DynamicCall(Vec<char>, Vec<Expr>),
    ListBuild(Box<Expr>, Box<Expr>),
    NumericExpr(numeric_expr::NumericExpr),
    LogicExpr(LogicExpr),
    Assign {
        pattern: Box<ArgBind>,
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
        Expr::Identifier(l) => match p.get(&l) {
            Some(v) => return v.clone(),
            _ => {
                println!("UNBOUND VAR{:#?}\n{:#?}\n{:#?}\n", p, expr, l);
                unimplemented!()
            }
        },
        Expr::ListBuild(h, t) => {
            let h1 = eval(c.clone(), p.clone(), *h);
            let t1 = eval(c.clone(), p.clone(), *t);
            return eval::Data::List(Box::new(h1), Box::new(t1));
        }
        Expr::DynamicCall(name, args) => match p.get(&name) {
            Some(eval::Data::FunctionPointer(fname)) => eval_and_call(c, fname.clone(), args, p),
            _ => {
                println!("NO FNAME{:#?}\n", expr);
                unimplemented!()
            }
        },
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
        _ => {
            println!("BAD EXPR{:#?}\n", expr);
            unimplemented!()
        }
    }
}

fn eval_and_call(
    c: eval::Program,
    f: function::FunctionName,
    args: Vec<Expr>,
    p: eval::ProgramState,
) -> eval::Data {
    let args1 = args
        .into_iter()
        .map(|arg| eval(c.clone(), p.clone(), arg))
        .collect();
    return eval::call(c, f, args1);
}

pub fn segments_to_expr(mut s: Vec<segments::Segment>) -> Expr {
    let res = call_levels::segments_to_call_level(s.clone());
    return call_levels_to_expr(res);
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
            /*let left1 = call_levels_to_expr(left);
            let right1 = call_levels_to_expr(right);
            let rest1 = call_levels_to_expr(rest);
            return Expr::Assign {
                pattern: Box::new(left1),
                arg: Box::new(right1),
                rest: Box::new(rest1),
            };*/
            println!("call_levels_to_expr {:#?}\n", level);
            unimplemented!()
        }
        _ => {
            println!("call_levels_to_expr {:#?}\n", level);
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
