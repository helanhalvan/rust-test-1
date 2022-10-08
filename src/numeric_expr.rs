use crate::eval::{self, Data};

#[derive(Debug, Clone)]
pub enum NumericExpr {
    Identifier(Vec<char>),
    Const(NumericData),
    Operator {
        op: NumOp,
        left: Box<NumericExpr>,
        right: Box<NumericExpr>,
    },
}

#[derive(Debug, Clone)]
pub enum NumOp {
    ADD,
    MUL,
    SUB,
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
        NumericExpr::Operator { op, left, right } => {
            let l1 = eval_int(c.clone(), p.clone(), *left);
            let r1 = eval_int(c.clone(), p.clone(), *right);
            match (op, l1.clone(), r1.clone()) {
                (NumOp::ADD, NumericData::Int(l2), NumericData::Int(r2)) => {
                    return NumericData::Int(l2 + r2);
                }
                (NumOp::SUB, NumericData::Int(l2), NumericData::Int(r2)) => {
                    return NumericData::Int(l2 - r2);
                }
                (NumOp::MUL, NumericData::Int(l2), NumericData::Int(r2)) => {
                    println!("MUL{:#?}{:#?}{:#?}{:#?}{:#?}\n", c, p, expr, l1, r1);
                    return NumericData::Int(l2 * r2);
                }
                _ => {
                    println!("NOT NUMBERS{:#?}{:#?}{:#?}{:#?}{:#?}\n", c, p, expr, l1, r1);
                    unimplemented!()
                }
            }
        }
        _ => {
            println!("BAD NEXPR{:#?}\n", expr);
            unimplemented!()
        }
    }
}
