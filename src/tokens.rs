#[derive(Debug)]
pub enum MyError {
    CannotParse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    AND,
    Add,
    ArgTerm,
    Assign,
    CodeEnd,
    CodeStart,
    CommentEnd,
    CommentStart,
    END,
    Eq,
    FunStart,
    Identifier(Vec<char>),
    LeftB,
    LeftP,
    LeftW,
    MUL,
    NEQ,
    Pipe,
    RightB,
    RightP,
    RightW,
    SUB,
}
// mapping left pair -> right pair
// ( -> )
pub fn has_pair(t: Token) -> Option<Token> {
    let p = match t {
        Token::CommentStart => Token::CommentEnd,
        Token::LeftP => Token::RightP,
        Token::LeftB => Token::RightB,
        Token::LeftW => Token::RightW,
        Token::CodeStart => Token::CodeEnd,
        Token::FunStart => Token::END,
        _ => return None,
    };
    return Some(p);
}

pub fn is_operator_token(t: Token) -> bool {
    match t {
        Token::Add | Token::Eq | Token::NEQ | Token::AND | Token::MUL | Token::SUB => true,
        _ => false,
    }
}

pub fn string_to(s: String) -> Vec<Token> {
    let mut ret: Vec<Token> = Vec::new();
    let text: Vec<char> = s.chars().collect();
    let mut token_head = 0;
    let mut identifier_head = 0;
    let mut identifier_tail = 0;
    loop {
        let maybe_token = if token_head >= (text.len() - 1) {
            return ret;
        } else if token_head + 1 == text.len() {
            single_char_to_token(text[token_head])
        } else if token_head + 2 == text.len() {
            two_chars_to_token(text[token_head], text[token_head + 1])
        } else {
            tre_chars_to_token(text[token_head], text[token_head + 1], text[token_head + 2])
        };
        match maybe_token {
            Some((t, ts)) => {
                if identifier_tail != token_head {
                    let id = text[identifier_tail..identifier_head].to_vec();
                    ret.push(Token::Identifier(id));
                }
                ret.push(t);
                token_head = token_head + (ts as usize); // dose not seem to increment
                identifier_tail = token_head;
                continue;
            }
            None => {
                token_head += 1;
                identifier_head = token_head;
            }
        }
    }
}

fn single_char_to_token(char: char) -> Option<(Token, u16)> {
    let ret: Token = match char {
        '+' => Token::Add,
        '(' => Token::LeftP,
        ')' => Token::RightP,
        '[' => Token::LeftB,
        ']' => Token::RightB,
        '{' => Token::LeftW,
        '}' => Token::RightW,
        ',' => Token::ArgTerm,
        '|' => Token::Pipe,
        '/' => Token::CodeStart,
        '\\' => Token::CodeEnd,
        ':' => Token::FunStart,
        '*' => Token::MUL,
        '-' => Token::SUB,
        '=' => Token::Assign,
        _ => {
            return None;
        }
    };
    return Some((ret, 1));
}

fn two_chars_to_token(a: char, b: char) -> Option<(Token, u16)> {
    let ret: Token = match (a, b) {
        ('/', '*') => Token::CommentStart,
        ('*', '/') => Token::CommentEnd,
        ('&', '&') => Token::AND,
        ('=', '=') => Token::Eq,
        _ => return single_char_to_token(a),
    };
    return Some((ret, 2));
}

fn tre_chars_to_token(a: char, b: char, c: char) -> Option<(Token, u16)> {
    match (a, b, c) {
        ('=', '/', '=') => {
            return Some((Token::NEQ, 3));
        }
        ('e', 'n', 'd') => {
            return Some((Token::END, 3));
        }
        _ => {
            return two_chars_to_token(a, b);
        }
    }
}
