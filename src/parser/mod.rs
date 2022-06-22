#![allow(dead_code)]
mod symbol;
#[cfg(test)]
mod tests;

use super::lexer::Token;
use crate::eval::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Nil,
    T,
    Symbol(String),
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>),
    Func {
        fun: fn(Sexp) -> std::result::Result<Sexp, RuntimeError>,
        name: &'static str,
    },
}

pub type Result<T> = std::result::Result<T, ParserError>;

pub fn read_from_tokens(tokens: Vec<Token>) -> Result<Vec<Sexp>> {
    let mut expressions: Vec<Sexp> = Vec::new();

    let mut tokens = tokens.into_iter();
    while let Some(token) = tokens.next() {
        match token {
            Token::RParen => {
                return Err(ParserError {
                    message: String::from("unexpected closing paren"),
                })
            }
            Token::LParen => {
                expressions.push(parse_list(&mut tokens)?);
            }
            Token::String(token) => {
                expressions.push(parse_string(token));
            }
            Token::Symbol(token) => expressions.push(parse_symbol(token)),
        }
    }

    Ok(expressions)
}

fn parse_string(token: String) -> Sexp {
    Sexp::Atom(Atom::String(token))
}

fn parse_symbol(token: String) -> Sexp {
    if let "t" | "T" | "true" = token.as_str() {
        return Sexp::Atom(Atom::T);
    }

    if let "nil" | "NIL" | "false" = token.as_str() {
        return Sexp::Atom(Atom::Nil);
    }

    if let Ok(f) = token.parse::<f64>() {
        return Sexp::Atom(Atom::Number(f));
    }

    Sexp::Atom(Atom::Symbol(token))
}

fn parse_list<I>(tokens: &mut I) -> Result<Sexp>
where
    I: Iterator<Item = Token>,
{
    let mut vec: Vec<Sexp> = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::RParen => {
                if vec.is_empty() {
                    return Ok(Sexp::Atom(Atom::Nil));
                }
                return Ok(Sexp::List(vec));
            }
            Token::LParen => {
                vec.push(parse_list(tokens)?);
            }
            Token::String(token) => {
                vec.push(parse_string(token));
            }
            Token::Symbol(token) => vec.push(parse_symbol(token)),
        }
    }
    Err(ParserError {
        // this should be unreachable due to lexer check
        message: String::from("missing closing parenthesis"),
    })
}
