pub mod expr;
pub mod function;
pub mod logic_expr;
pub mod program;
pub mod segments;
pub mod tokens;

use std::{collections::HashMap, fs, println};

use segments::{Clause, Segment}; //, vec

fn main() {
    let c = fs::read_to_string("./program.txt").expect("Cant read file");
    let c2 = clean(c);
    let tokens = tokens::string_to(c2.clone());
    // Groups of tokes for expressing a single case within the code base
    let clauses = segments::matched_terminators(tokens.clone());
    let clauses2 = segments::prune_clauses(clauses.clone());
    let funs = program::to_funs(clauses2.clone());
    println!("9{:#?}\n", (funs));
    let res = function::call(funs, "main".chars().collect(), Vec::new());
    println!("DONE:{:#?}\n", res);
}

fn clean(s: String) -> String {
    return s.chars().filter(|c| !c.is_whitespace()).collect();
}
