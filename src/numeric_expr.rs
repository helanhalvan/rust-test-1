use crate::{
    call_levels,
    eval::{self, Data},
    expr,
    tokens::Token,
};

#[derive(Debug, Clone)]
pub enum NumericExpr {
    Identifier(Vec<char>),
    Call(Vec<char>, Vec<expr::Expr>),
    DynamicCall(Vec<char>, Vec<expr::Expr>),
    Const(NumericData),
    ArrayOperator {
        op: ArrayNumOp,
        v: Vec<NumericExpr>,
    },
    OrderedOperator {
        op: OrderedNumOp,
        left: Box<NumericExpr>,
        right: Box<NumericExpr>,
    },
}

#[derive(Debug, Clone)]
pub enum ArrayNumOp {
    ADD,
    MUL,
}

#[derive(Debug, Clone)]
pub enum OrderedNumOp {
    SUB,
    DIV,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumericData {
    Int(i64),
}

pub fn eval(c: eval::Program, p: eval::ProgramState, expr: NumericExpr) -> eval::Data {
    let res = eval_int(c, p, expr);
    return eval::Data::Number(res);
}

pub fn eval_int(c: eval::Program, p: eval::ProgramState, expr: NumericExpr) -> NumericData {
    match expr.clone() {
        NumericExpr::Const(c) => {
            return c;
        }
        NumericExpr::Identifier(l) => match p.get(&l) {
            Some(eval::Data::Number(v)) => return v.clone(),
            bad => {
                println!("UNBOUND VAR{:#?}\n{:#?}\n{:#?}\n{:#?}\n", p, expr, l, bad);
                unimplemented!()
            }
        },
        NumericExpr::OrderedOperator { op, left, right } => {
            let l1 = eval_int(c.clone(), p.clone(), *left);
            let r1 = eval_int(c.clone(), p.clone(), *right);
            match (op, l1.clone(), r1.clone()) {
                (OrderedNumOp::SUB, NumericData::Int(l2), NumericData::Int(r2)) => {
                    return NumericData::Int(l2 - r2);
                }
                _ => {
                    println!("NOT NUMBERS{:#?}{:#?}{:#?}{:#?}{:#?}\n", c, p, expr, l1, r1);
                    unimplemented!()
                }
            }
        }
        NumericExpr::ArrayOperator { op, v } => {
            let vc = v.iter().map(|x| {
                let NumericData::Int(i) = eval_int(c.clone(), p.clone(), x.clone());
                i
            });
            match op {
                ArrayNumOp::ADD => {
                    return NumericData::Int(vc.sum());
                }
                ArrayNumOp::MUL => {
                    return NumericData::Int(vc.fold(1, |a, b| a * b));
                }
            }
        }
        NumericExpr::Call(f, args) => match expr::eval_and_call(c, f, args, p) {
            eval::Data::Number(a) => a,
            a => {
                println!("BAD NEXPR{:#?}\n", a);
                unimplemented!()
            }
        },
        _ => {
            println!("BAD NEXPR{:#?}\n", expr);
            unimplemented!()
        }
    }
}

pub fn call_levels_to_num_expr(level: call_levels::CallLevel) -> NumericExpr {
    match level.clone() {
        call_levels::CallLevel::OpLevel(token, sublevels) => {
            let subs = sublevels
                .into_iter()
                .map(|x| call_levels_to_num_expr(x))
                .collect();
            match token {
                Token::Add => {
                    return NumericExpr::ArrayOperator {
                        op: ArrayNumOp::ADD,
                        v: subs,
                    }
                }
                Token::MUL => {
                    return NumericExpr::ArrayOperator {
                        op: ArrayNumOp::MUL,
                        v: subs,
                    }
                }
                Token::SUB => match (subs.get(0), subs.get(1), subs.get(2)) {
                    (Some(left), Some(right), None) => {
                        return NumericExpr::OrderedOperator {
                            op: OrderedNumOp::SUB,
                            left: Box::new(left.clone()),
                            right: Box::new(right.clone()),
                        };
                    }
                    t => {
                        println!("call_levels_to_expr {:#?}{:#?}\n", level, t);
                        unimplemented!()
                    }
                },
                t => {
                    println!("call_levels_to_expr {:#?}{:#?}\n", level, t);
                    unimplemented!()
                }
            }
        }
        call_levels::CallLevel::Identifier(v) => return string_token_to_num_expr(v),
        call_levels::CallLevel::Call(fname, sublevels) => {
            let subs = sublevels
                .into_iter()
                .map(|x| expr::call_levels_to_expr(x))
                .collect();
            return NumericExpr::Call(fname, subs);
        }
        _ => {
            println!("call_levels_to_expr {:#?}\n", level);
            unimplemented!()
        }
    }
}

pub fn string_token_to_num_expr(chars: Vec<char>) -> NumericExpr {
    let text = String::from_iter(chars.iter());
    if let Ok(n) = text.parse::<i64>() {
        return NumericExpr::Const(NumericData::Int(n));
    } else {
        return NumericExpr::Identifier(chars);
    }
}
