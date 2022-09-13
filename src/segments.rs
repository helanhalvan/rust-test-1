use crate::tokens::Token;

#[derive(Debug, Clone)]
pub struct Clause {
    pub head: crate::tokens::Token,
    pub body: Vec<Segment>,
    pub tail: Token,
}

#[derive(Debug, Clone)]
pub enum Segment {
    Clause(Clause),
    UnMatched(Vec<Token>),
}

pub fn prune_clauses(t: Vec<Segment>) -> Vec<Segment> {
    let x = t
        .iter()
        .filter(|i| match i {
            Segment::Clause(Clause {
                head: Token::CommentStart,
                ..
            }) => false,
            Segment::UnMatched(l) => l.len() > 0,
            _ => true,
        })
        .map(|i| match i {
            Segment::Clause(Clause {
                head: h,
                tail: t,
                body: b,
            }) => Segment::Clause(Clause {
                head: h.clone(),
                tail: t.clone(),
                body: prune_clauses(b.clone()),
            }),
            A => A.clone(),
        })
        .collect();
    return x;
}

//recursively match terminators until there are no terminators left
pub fn matched_terminators(t: Vec<crate::tokens::Token>) -> Vec<Segment> {
    return clauses_int(t, Vec::new(), Vec::new());
}

fn clauses_int(
    t: Vec<crate::tokens::Token>,
    mut done: Vec<Segment>,
    mut acc: Vec<Token>,
) -> Vec<Segment> {
    match t.len() {
        0 => {
            done.push(Segment::UnMatched(acc));
            return done;
        }
        _ => {
            let rest = t[1..t.len()].to_vec();
            match crate::tokens::has_pair(t[0].clone()) {
                Some(p) => {
                    done.push(Segment::UnMatched(acc));
                    let (clause, t) = build_clause(t[0].clone(), p, Vec::new(), rest, 0);
                    done.push(Segment::Clause(clause));
                    return clauses_int(t, done, Vec::new());
                }
                _ => {
                    acc.push(t[0].clone());
                    return clauses_int(rest, done, acc);
                }
            }
        }
    }
}

fn build_clause(
    head: Token,
    tail: Token,
    mut body: Vec<Token>,
    t: Vec<crate::tokens::Token>,
    depth: usize,
) -> (Clause, Vec<Token>) {
    let rest = if t.len() > 1 {
        t[1..t.len()].to_vec()
    } else {
        Vec::new()
    };
    if t[0] == tail && depth == 0 {
        let b2 = matched_terminators(body);
        return (
            Clause {
                head: head,
                tail: tail,
                body: b2,
            },
            rest,
        );
    } else if t[0] == head {
        body.push(t[0].clone());
        return build_clause(head, tail, body, rest, depth + 1);
    } else if t[0] == tail {
        body.push(t[0].clone());
        return build_clause(head, tail, body, rest, depth - 1);
    } else {
        body.push(t[0].clone());
        return build_clause(head, tail, body, rest, depth);
    }
}
