use crate::call_levels;
use crate::eval::{self, ProgramState};
use crate::expr::{self, Expr};
use crate::segments::{self, Segment};
use crate::tokens::{self, Token};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum LogicExpr {
    True,
    False,
    Identifier(Vec<char>),
    AND(Vec<LogicExpr>),
    EQ(Vec<expr::Expr>),
    NEQ(Box<expr::Expr>, Box<expr::Expr>),
    Call(Vec<char>, Vec<Expr>),
    DynamicCall(Vec<char>, Vec<Expr>),
}

pub fn eval(c: eval::Program, p: eval::ProgramState, expr: LogicExpr) -> bool {
    match expr {
        LogicExpr::False => false,
        LogicExpr::True => true,
        LogicExpr::NEQ(l, r) => {
            let l1 = expr::eval(c.clone(), p.clone(), *l);
            let r1 = expr::eval(c, p, *r);
            l1 != r1
        }
        LogicExpr::EQ(v) => {
            let res = v
                .into_iter()
                .map(|x| expr::eval(c.clone(), p.clone(), x))
                .all_equal();
            res
        }
        LogicExpr::AND(v) => {
            let res = v
                .into_iter()
                .map(|x| eval(c.clone(), p.clone(), x))
                .all(|x| x);
            res
        }
        _ => {
            println!("logic_expr{:#?}{:#?}\n", p, expr);
            unimplemented!()
        }
    }
}

pub fn segments_to_logical_expr(s: Vec<segments::Segment>) -> LogicExpr {
    let res = call_levels::segments_to_call_level(s.clone());
    return call_levels_to_logic_expr(res);
}

pub fn call_levels_to_logic_expr(level: call_levels::CallLevel) -> LogicExpr {
    match level.clone() {
        call_levels::CallLevel::OpLevel(token, sublevels) => match token {
            Token::Eq => {
                let subs = sublevels
                    .into_iter()
                    .map(|x| expr::call_levels_to_expr(x))
                    .collect();
                return LogicExpr::EQ(subs);
            }
            t => {
                println!("call_levels_to_expr {:#?}{:#?}\n", level, t);
                unimplemented!()
            }
        },
        call_levels::CallLevel::Identifier(v) => return string_token_to_logic_expr(v),
        call_levels::CallLevel::Call(fname, sublevels) => {
            let subs = sublevels
                .into_iter()
                .map(|x| expr::call_levels_to_expr(x))
                .collect();
            return LogicExpr::Call(fname, subs);
        }
        _ => {
            println!("call_levels_to_expr {:#?}\n", level);
            unimplemented!()
        }
    }
}

pub fn string_token_to_logic_expr(chars: Vec<char>) -> LogicExpr {
    let text = String::from_iter(chars.iter());
    if let Ok(n) = text.parse::<bool>() {
        if n {
            return LogicExpr::True;
        } else {
            return LogicExpr::False;
        };
    } else {
        return LogicExpr::Identifier(chars);
    }
}
