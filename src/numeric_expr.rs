use crate::eval::{self, Data};

#[derive(Debug, Clone)]
pub enum NumericExpr {
    Identifier(Vec<char>),
    Int(u64),
    ADD(Box<NumericExpr>, Box<NumericExpr>),
    MUL(Box<NumericExpr>, Box<NumericExpr>),
    SUB(Box<NumericExpr>, Box<NumericExpr>),
}

pub fn eval(c: eval::Program, p: eval::ProgramState, expr: NumericExpr) -> eval::Data {
    match expr.clone() {
        NumericExpr::Int(c) => {
            return eval::Data::Number(c);
        }
        NumericExpr::Identifier(l) => match p.get(&l) {
            Some(v) => return v.clone(),
            _ => {
                println!("UNBOUND VAR{:#?}\n{:#?}\n{:#?}\n", p, expr, l);
                unimplemented!()
            }
        },
        NumericExpr::ADD(l, r) => {
            let l1 = eval(c.clone(), p.clone(), *l);
            let r1 = eval(c.clone(), p.clone(), *r);
            match (l1.clone(), r1.clone()) {
                (eval::Data::Number(l2), eval::Data::Number(r2)) => {
                    return Data::Number(l2 + r2);
                }
                _ => {
                    println!("NOT NUMBERS{:#?}{:#?}{:#?}{:#?}{:#?}\n", c, p, expr, l1, r1);
                    unimplemented!()
                }
            }
        }
        NumericExpr::SUB(l, r) => {
            let l1 = eval(c.clone(), p.clone(), *l);
            let r1 = eval(c.clone(), p.clone(), *r);
            match (l1.clone(), r1.clone()) {
                (eval::Data::Number(l2), eval::Data::Number(r2)) => {
                    return Data::Number(l2 - r2);
                }
                _ => {
                    println!("NOT NUMBERS{:#?}{:#?}{:#?}{:#?}{:#?}\n", c, p, expr, l1, r1);
                    unimplemented!()
                }
            }
        }
        NumericExpr::MUL(l, r) => {
            let l1 = eval(c.clone(), p.clone(), *l);
            let r1 = eval(c.clone(), p.clone(), *r);
            match (l1.clone(), r1.clone()) {
                (eval::Data::Number(l2), eval::Data::Number(r2)) => {
                    return Data::Number(l2 * r2);
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
