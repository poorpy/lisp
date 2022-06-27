mod rewrite;

#[cfg(test)]
mod tests;

use std::collections::VecDeque;

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
    List(VecDeque<Sexp>),
    Func {
        fun: fn(VecDeque<Sexp>) -> std::result::Result<Sexp, RuntimeError>,
        name: &'static str,
    },
}

impl Sexp {
    pub fn get_kind_name(&self) -> &str {
        match &self {
            Sexp::Atom(_) => "atom",
            Sexp::List(_) => "list",
            Sexp::Func { .. } => "builtin function",
        }
    }
}

impl From<Vec<Sexp>> for Sexp {
    fn from(vec: Vec<Sexp>) -> Self {
        Sexp::List(VecDeque::from(vec))
    }
}

pub type Result<T> = std::result::Result<T, ParserError>;

pub fn read_from_tokens(tokens: Vec<Token>) -> Result<VecDeque<Sexp>> {
    let mut expressions: VecDeque<Sexp> = VecDeque::new();

    let mut tokens = tokens.into_iter();
    while let Some(token) = tokens.next() {
        match token {
            Token::RParen => {
                return Err(ParserError {
                    message: String::from("unexpected closing paren"),
                })
            }
            Token::LParen => {
                expressions.push_back(parse_list(&mut tokens)?);
            }
            Token::String(token) => {
                expressions.push_back(parse_string(token));
            }
            Token::Symbol(token) => expressions.push_back(parse_symbol(token)),
        }
    }

    Ok(rewrite::rewrite_quotes(expressions))
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
    let mut vec: VecDeque<Sexp> = VecDeque::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::RParen => {
                if vec.is_empty() {
                    return Ok(Sexp::Atom(Atom::Nil));
                }
                return Ok(Sexp::List(vec));
            }
            Token::LParen => {
                vec.push_back(parse_list(tokens)?);
            }
            Token::String(token) => {
                vec.push_back(parse_string(token));
            }
            Token::Symbol(token) => vec.push_back(parse_symbol(token)),
        }
    }
    Err(ParserError {
        // this should be unreachable due to lexer check
        message: String::from("missing closing parenthesis"),
    })
}
