#[cfg(test)]
mod tests;

use crate::eval::{Result, RuntimeError};
use crate::parser::{Atom, Sexp};

/// Returns arguments as Sexp::List
pub fn list(sexp: Sexp) -> Result<Sexp> {
    if let Sexp::List(args) = sexp {
        if args.is_empty() {
            return Err(RuntimeError::WrongArgumentArity(args.len()));
        }

        return Ok(Sexp::List(args));
    }
    Err(RuntimeError::WrongArgumentKind(format!(
        "expected argument list instead got: {}",
        sexp.get_kind_name()
    )))
}

pub fn car(sexp: Sexp) -> Result<Sexp> {
    if let Sexp::List(args) = &sexp {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArgumentArity(args.len()));
        }

        if let Sexp::Atom(Atom::Nil) = args[0] {
            return Ok(Sexp::Atom(Atom::Nil));
        }

        if let Sexp::List(list) = &args[0] {
            if list.is_empty() {
                return Ok(Sexp::Atom(Atom::Nil));
            }

            return Ok(list[0].clone());
        }

        return Err(RuntimeError::WrongArgumentKind(format!(
            "car expects list as an argument instead got: {}",
            sexp.get_kind_name()
        )));
    }

    Err(RuntimeError::WrongArgumentKind(format!(
        "expected argument list instead got: {}",
        sexp.get_kind_name()
    )))
}

pub fn cdr(sexp: Sexp) -> Result<Sexp> {
    if let Sexp::List(args) = &sexp {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArgumentArity(args.len()));
        }

        if let Sexp::Atom(Atom::Nil) = args[0] {
            return Ok(Sexp::Atom(Atom::Nil));
        }

        if let Sexp::List(list) = &args[0] {
            if list.is_empty() {
                return Ok(Sexp::Atom(Atom::Nil));
            }

            return Ok(Sexp::List(list[1..].to_vec()));
        }

        return Err(RuntimeError::WrongArgumentKind(format!(
            "cdr expects list as an argument instead got: {}",
            sexp.get_kind_name()
        )));
    }

    Err(RuntimeError::WrongArgumentKind(format!(
        "expected argument list instead got: {}",
        sexp.get_kind_name()
    )))
}
