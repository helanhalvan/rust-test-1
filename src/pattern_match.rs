use crate::{
    call_levels, eval, expr,
    segments::{self, Segment},
    tokens,
};

// target of a pattern match (such as a function argument or assignment left hand)
#[derive(Debug, Clone)]
pub enum ArgBind {
    Emptylist,
    ListPattern {
        head: Box<ArgBind>,
        tail: Box<ArgBind>,
    },
    ConstPattern(eval::Data),
    Identifier(Vec<char>),
}

pub fn call_level_to_argbind(level: call_levels::CallLevel) -> ArgBind {
    match level {
        call_levels::CallLevel::Identifier(i) => {
            return ArgBind::Identifier(i);
        }
        _ => {
            println!("BAD_BINDSEG{:#?}\n", level);
            unimplemented!()
        }
    }
}

pub fn bind_segment_to_bindpattern(s: Vec<Segment>) -> Vec<ArgBind> {
    if s.len() == 0 {
        return Vec::new();
    }
    if s.len() == 1 {
        return single_seg_to_bind(s[0].clone());
    }
    if s.len() == 2 {
        match s[1].clone() {
            segments::Segment::UnMatched(t) => match t.split_first() {
                Some((tokens::Token::ArgTerm, rest)) => {
                    let mut res = single_seg_to_bind(s[0].clone());
                    let mut more = bind_tokens_to_bindpattern(rest.to_vec());
                    res.append(&mut more);
                    return res;
                }
                a => {
                    println!("{:#?}\n", a);
                    unimplemented!()
                }
            },
            a => {
                println!("{:#?}\n", a);
                unimplemented!()
            }
        }
    }
    println!("BAD_BINDSEG{:#?}\n", s);
    unimplemented!()
}

fn single_seg_to_bind(seg: Segment) -> Vec<ArgBind> {
    match seg {
        // (A), (A, B), etc
        segments::Segment::UnMatched(i) => return bind_tokens_to_bindpattern(i),
        // []
        // [H|T], etc
        segments::Segment::Clause {
            head: tokens::Token::LeftB,
            body: i,
            ..
        } => {
            if i.len() == 0 {
                let mut res = Vec::new();
                res.push(ArgBind::ConstPattern(eval::Data::Emptylist));
                return res;
            } else {
                match &i[0] {
                    segments::Segment::UnMatched(t) => {
                        if t.len() == 3 && t[1] == tokens::Token::Pipe {
                            let mut res = Vec::new();
                            let head = token_to_bind(t[0].clone());
                            let tail = token_to_bind(t[2].clone());
                            res.push(ArgBind::ListPattern {
                                head: Box::new(head),
                                tail: Box::new(tail),
                            });
                            return res;
                        }
                        println!("1{:#?}\n", t);
                        unimplemented!()
                    }
                    seg => {
                        println!("2{:#?}\n", seg);
                        unimplemented!()
                    }
                }
            }
        }
        seg => {
            println!("3{:#?}\n", seg);
            unimplemented!()
        }
    };
}

fn bind_tokens_to_bindpattern(s: Vec<tokens::Token>) -> Vec<ArgBind> {
    if s.len() == 0 {
        return Vec::new();
    }
    if s.len() == 1 {
        let bind = token_to_bind(s[0].clone());
        let mut args = Vec::new();
        args.insert(0, bind);
        return args;
    }
    // handing (A , ..)
    match s[1].clone() {
        tokens::Token::ArgTerm => {
            let bind = token_to_bind(s[0].clone());
            let rest = s[2..s.len()].to_vec();
            let mut args = bind_tokens_to_bindpattern(rest);
            args.insert(0, bind);
            return args;
        }
        _ => unimplemented!(),
    }
}

fn token_to_bind(t: tokens::Token) -> ArgBind {
    if let Some(c) = try_token_to_constant(t.clone()) {
        return c;
    } else if let Some(i) = try_token_to_identifier(t) {
        return i;
    } else {
        unimplemented!();
    };
}

fn try_token_to_identifier(t: tokens::Token) -> Option<ArgBind> {
    match t {
        tokens::Token::Identifier(c) => return Some(ArgBind::Identifier(c)),
        _ => return None,
    }
}

fn try_token_to_constant(t: tokens::Token) -> Option<ArgBind> {
    match t {
        // TODO handle [] etc
        _ => {
            return None;
        }
    }
}
