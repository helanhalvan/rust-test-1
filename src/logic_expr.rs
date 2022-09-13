use crate::eval::{self, ProgramState};
use crate::segments::{self, Clause, Segment};
use crate::tokens::{self, Token};

#[derive(Debug, Clone)]
pub enum LogicExpr {
    True,
    False,
    Identifier(Vec<char>),
    Eq(Box<LogicExpr>, Box<LogicExpr>),
    NEQ(Box<LogicExpr>, Box<LogicExpr>),
    AND(Box<LogicExpr>, Box<LogicExpr>),
}

pub fn eval(p: eval::ProgramState, expr: LogicExpr) -> bool {
    match expr {
        LogicExpr::False => false,
        LogicExpr::True => true,
        _ => {
            println!("logic_expr{:#?}{:#?}\n", p, expr);
            unimplemented!()
        }
    }
}

pub fn segments_to_logical_expr(s: Vec<segments::Segment>) -> LogicExpr {
    if s.len() == 1 {
        match s[0].clone() {
            segments::Segment::UnMatched(tv) => return tokens_to_logical_expr(tv.clone()),
            _ => {
                println!("ALONE{:#?}\n", s[0]);
                unimplemented!()
            }
        }
    }
    if s.len() == 3 {
        match (s[0].clone(), s[1].clone(), s[2].clone()) {
            (
                segments::Segment::Clause(Clause {
                    head: tokens::Token::LeftP,
                    body: sv,
                    ..
                }),
                segments::Segment::UnMatched(um),
                segments::Segment::Clause(Clause {
                    head: tokens::Token::LeftP,
                    body: sv2,
                    ..
                }),
            ) => {
                if um.len() == 1 {
                    match um[0] {
                        Token::AND => {
                            let left = segments_to_logical_expr(sv.clone());
                            let right = segments_to_logical_expr(sv2.to_vec());
                            return LogicExpr::AND(Box::new(left), Box::new(right));
                        }
                        _ => {
                            println!("{:#?}\n", s);
                            unimplemented!()
                        }
                    }
                } else {
                    println!("{:#?}\n", s);
                    unimplemented!()
                }
            }
            bad => {
                println!("{:#?}{:#?}\n", s, bad);
                unimplemented!()
            }
        }
    }
    // TODO handle longer logic expr like
    // a && b && c && d
    // not sure what () rules will be needed
    println!("{:#?}\n", s);
    unimplemented!()
}

fn tokens_to_logical_expr(tv: Vec<Token>) -> LogicExpr {
    match tv.len() {
        1 => {
            println!("1{:#?}\n", tv);
            unimplemented!()
        }
        3 => match (tv[0].clone(), tv[1].clone(), tv[2].clone()) {
            (Token::Identifier(l), Token::Eq, Token::Identifier(r)) => {
                return LogicExpr::Eq(
                    Box::new(LogicExpr::Identifier(l)),
                    Box::new(LogicExpr::Identifier(r)),
                )
            }
            (Token::Identifier(l), Token::NEQ, Token::Identifier(r)) => {
                return LogicExpr::NEQ(
                    Box::new(LogicExpr::Identifier(l)),
                    Box::new(LogicExpr::Identifier(r)),
                )
            }
            _ => {
                println!("3{:#?}\n", tv);
                unimplemented!()
            }
        },
        _ => {
            println!("{:#?}\n", tv);
            unimplemented!()
        }
    }
}
