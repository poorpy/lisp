#[cfg(test)]
mod tests;

use std::collections::VecDeque;

use crate::eval::{Result, RuntimeError};
use crate::parser::{Atom, Sexp};

/// Returns arguments as `Sexp::List`
pub fn list(args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    Ok(Sexp::List(args))
}

/// Returns head (first argument) of a list
pub fn car(mut args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    let sexp = args.pop_front().unwrap();

    if let Sexp::Atom(Atom::Nil) = sexp {
        return Ok(Sexp::Atom(Atom::Nil));
    }

    if let Sexp::List(mut list) = sexp {
        if list.is_empty() {
            return Ok(Sexp::Atom(Atom::Nil));
        }

        return Ok(list.pop_front().unwrap());
    }

    Err(RuntimeError::WrongArgumentKind(format!(
        "car expects list as an argument instead got: {}",
        sexp.get_kind_name()
    )))
}

/// Returns tail (all but first argument) of list
pub fn cdr(mut args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    let sexp = args.pop_front().unwrap();

    if let Sexp::Atom(Atom::Nil) = sexp {
        return Ok(Sexp::Atom(Atom::Nil));
    }

    if let Sexp::List(mut list) = sexp {
        if list.is_empty() {
            return Ok(Sexp::Atom(Atom::Nil));
        }

        list.pop_front();

        return Ok(Sexp::List(list));
    }

    Err(RuntimeError::WrongArgumentKind(format!(
        "cdr expects list as an argument instead got: {}",
        sexp.get_kind_name()
    )))
}

// Returns new slice with second arg pushed to back
pub fn append(mut args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    let sexp = args.pop_front().unwrap();
    let appendee = args.pop_front().unwrap();

    if let Sexp::List(mut list) = sexp {
        list.push_back(appendee);
        return Ok(Sexp::List(list));
    }

    if let Sexp::Atom(Atom::Nil) = sexp {
        return Ok(Sexp::from(vec![appendee]));
    }

    Err(RuntimeError::WrongArgumentKind(format!(
        "append expects list as first argument instead got: {}",
        sexp.get_kind_name()
    )))
}

// Returns new slice with second arg pushed to front
pub fn prepend(mut args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    let sexp = args.pop_front().unwrap();
    let appendee = args.pop_front().unwrap();

    if let Sexp::List(mut list) = sexp {
        list.push_front(appendee);
        return Ok(Sexp::List(list));
    }

    if let Sexp::Atom(Atom::Nil) = sexp {
        return Ok(Sexp::from(vec![appendee]));
    }

    Err(RuntimeError::WrongArgumentKind(format!(
        "append expects list as first argument instead got: {}",
        sexp.get_kind_name()
    )))
}

/// Returns unevaluated argument
pub fn quote(mut args: VecDeque<Sexp>) -> Result<Sexp> {
    if args.len() != 1 {
        return Err(RuntimeError::WrongArgumentArity(args.len()));
    }

    Ok(args.pop_back().unwrap())
}
