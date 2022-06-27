#[cfg(test)]
mod tests;

use std::collections::VecDeque;

use crate::eval::{Result, RuntimeError};
use crate::parser::{Atom, Sexp};


pub fn is_atom(args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    match args[0] {
        Sexp::Atom(_) => Ok(Sexp::Atom(Atom::T)),
        _ => Ok(Sexp::Atom(Atom::Nil)),
    }
}

pub fn is_list(args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    match args[0] {
        Sexp::List(_) | Sexp::Atom(Atom::Nil) => Ok(Sexp::Atom(Atom::T)),
        _ => Ok(Sexp::Atom(Atom::Nil)),
    }
}

pub fn is_function(args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    match args[0] {
        Sexp::Func { .. } => Ok(Sexp::Atom(Atom::T)),
        _ => Ok(Sexp::Atom(Atom::Nil)),
    }
}

pub fn is_string(args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    match args[0] {
        Sexp::Atom(Atom::String(_)) => Ok(Sexp::Atom(Atom::T)),
        _ => Ok(Sexp::Atom(Atom::Nil)),
    }
}

pub fn is_symbol(args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    match args[0] {
        Sexp::Atom(Atom::Symbol(_)) => Ok(Sexp::Atom(Atom::T)),
        _ => Ok(Sexp::Atom(Atom::Nil)),
    }
}

pub fn is_number(args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    match args[0] {
        Sexp::Atom(Atom::Number(_)) => Ok(Sexp::Atom(Atom::T)),
        _ => Ok(Sexp::Atom(Atom::Nil)),
    }
}
