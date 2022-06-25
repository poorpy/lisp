#[cfg(test)]
mod tests;

use crate::eval::{Result, RuntimeError};
use crate::parser::Sexp;

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

// pub fn car(sexp: Sexp) -> Result<Sexp> {

// }
