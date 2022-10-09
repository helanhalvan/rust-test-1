use std::{borrow::Borrow, collections::HashSet};

use crate::{
    eval::{self, Program},
    expr::Expr,
    logic_expr::LogicExpr,
    program::{self, Fun},
};

// TODO make atom table for function names

pub type FunctionName = Vec<char>;

type FunctionNames = HashSet<Vec<char>>;

pub fn resolve_lambdas(p1: eval::Program) -> eval::Program {
    let function_names: FunctionNames = p1.keys().map(|f| f.to_vec()).collect();
    let p: Program = p1
        .clone()
        .iter()
        .map(|(key, f1)| {
            let program::Fun { binds, .. } = f1;
            let b1 = binds
                .into_iter()
                .map(|b1| {
                    let program::Bind { filters, .. } = b1;
                    let f1 = filters
                        .into_iter()
                        .map(|f| {
                            let program::Filter { head, code } = f;
                            let h1 = resolve_lambdas_lexpr(function_names.clone(), head.clone());
                            let c1 = resolve_lambdas_expr(function_names.clone(), code.clone());
                            program::Filter { head: h1, code: c1 }
                        })
                        .collect();
                    program::Bind {
                        filters: f1,
                        ..b1.clone()
                    }
                })
                .collect();
            (
                key.to_vec(),
                program::Fun {
                    binds: b1,
                    ..f1.clone()
                },
            )
        })
        .collect();
    println!("{:#?}{:#?}{:#?}\n", p1, function_names, p);
    return p;
}

fn resolve_lambdas_expr(function_names: FunctionNames, e: Expr) -> Expr {
    let org = e.clone();
    match e {
        Expr::Call(id, body) => {
            if function_names.contains(&id) {
                let b1: Vec<Expr> = body
                    .iter()
                    .map(|item| resolve_lambdas_expr(function_names.clone(), item.clone()))
                    .collect();
                return Expr::Call(id, b1);
            } else {
                return Expr::DynamicCall(id, body);
            }
        }
        Expr::Identifier(id) => {
            if function_names.contains(&id) {
                return Expr::Constant(eval::Data::FunctionPointer(id));
            } else {
                return org;
            }
        }
        Expr::ListBuild(a, b) => {
            return Expr::ListBuild(
                Box::new(resolve_lambdas_expr(function_names.clone(), *a)),
                Box::new(resolve_lambdas_expr(function_names, *b)),
            )
        }
        Expr::Constant(_) => {
            return org;
        }
        Expr::LogicExpr(l) => {
            return Expr::LogicExpr(resolve_lambdas_lexpr(function_names, l));
        }
        Expr::NumericExpr(n) => {
            // TODO handle functions in numeric expr
            return org;
        }
        Expr::Assign { arg, rest, pattern } => {
            return Expr::Assign {
                arg: Box::new(resolve_lambdas_expr(function_names.clone(), *arg)),
                rest: Box::new(resolve_lambdas_expr(function_names, *rest)),
                pattern: pattern,
            };
        }
        _ => {
            println!("{:#?}\n", e);
            unimplemented!()
        }
    };
}

fn resolve_lambdas_lexpr(function_names: FunctionNames, e: LogicExpr) -> LogicExpr {
    let org = e.clone();
    match e {
        LogicExpr::True => org,
        LogicExpr::False => org,
        LogicExpr::AND(v) => {
            //let l1 = resolve_lambdas_lexpr(function_names.clone(), *l);
            //let r1 = resolve_lambdas_lexpr(function_names, *r);
            return LogicExpr::AND(v);
        }
        LogicExpr::EQ(v) => {
            //let l1 = resolve_lambdas_expr(function_names.clone(), *l);
            //let r1 = resolve_lambdas_expr(function_names, *r);
            return LogicExpr::EQ(v);
        }
        LogicExpr::NEQ(l, r) => {
            let l1 = resolve_lambdas_expr(function_names.clone(), *l);
            let r1 = resolve_lambdas_expr(function_names, *r);
            return LogicExpr::NEQ(Box::new(l1), Box::new(r1));
        }
        LogicExpr::Call(id, body) => {
            if function_names.contains(&id) {
                let b1: Vec<Expr> = body
                    .iter()
                    .map(|item| resolve_lambdas_expr(function_names.clone(), item.clone()))
                    .collect();
                return LogicExpr::Call(id, b1);
            } else {
                return LogicExpr::DynamicCall(id, body);
            }
        }
        _ => {
            println!("{:#?}\n", e);
            unimplemented!()
        }
    }
}
