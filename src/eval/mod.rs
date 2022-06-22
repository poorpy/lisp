#![allow(dead_code)]
#[cfg(test)]
mod tests;

use super::parser::{Sexp, Atom};
use super::env::{Env, LookupError};

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    Any,
    UndefinedSymbol(String)
}

impl From<LookupError> for RuntimeError {
    fn from(err: LookupError) -> Self {
        RuntimeError::UndefinedSymbol(err.message)
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub fn eval(sexp: Sexp, env: &mut Env) -> Result<Sexp> {
    match sexp {
        Sexp::Atom(atom) => eval_atom(atom, env),
        _ => unimplemented!()
    }
}

fn eval_atom(atom: Atom, env: &Env) -> Result<Sexp> {
    match atom {
        Atom::Symbol(s) => Ok(env.search(s)?),
        _ => Ok(Sexp::Atom(atom))
    }
}
