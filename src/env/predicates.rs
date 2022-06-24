use crate::eval::{Result, RuntimeError};
use crate::parser::{Atom, Sexp};

pub fn is_atom(sexp: Sexp) -> Result<Sexp> {
    if let Sexp::List(args) = sexp {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArgumentArity(args.len()));
        }

        match args[0] {
            Sexp::Atom(_) => return Ok(Sexp::Atom(Atom::T)),
            _ => return Ok(Sexp::Atom(Atom::Nil)),
        }
    }

    Err(RuntimeError::WrongArgumentKind(format!(
        "expected argument list instead got: {}",
        sexp.get_kind_name()
    )))
}
