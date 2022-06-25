#![allow(dead_code)]
#[cfg(test)]
mod tests;

use super::env::{Env, LookupError};
use super::parser::{Atom, Sexp};

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    Any,
    WrongArgumentArity(usize),
    WrongArgumentKind(String),
    UndefinedSymbol(String),
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
        f @ Sexp::Func { .. } => Ok(f),
        Sexp::List(list) => eval_list(list, env),
    }
}

fn eval_atom(atom: Atom, env: &Env) -> Result<Sexp> {
    match atom {
        Atom::Symbol(s) => Ok(env.search(s)?),
        _ => Ok(Sexp::Atom(atom)),
    }
}

fn eval_list(mut list: Vec<Sexp>, env: &mut Env) -> Result<Sexp> {
    if list.is_empty() {
        return Ok(Sexp::Atom(Atom::Nil));
    }

    match &list[0] {
        Sexp::Func { fun, .. } => {
            let mut args: Vec<Sexp> = Vec::new();
            for item in list[1..].iter().map(|sexp| eval(sexp.clone(), env)) {
                args.push(item?)
            }
            apply_builtin(*fun, args)
        }
        Sexp::Atom(atom @ Atom::Symbol(_)) => {
            // we replace initial symbol with its expanded form
            list[0] = eval_atom(atom.clone(), env)?;
            eval_list(list, env)
        }
        _ => Err(RuntimeError::WrongArgumentKind(format!(
            "eval list, expected function, got: {}",
            list[0].get_kind_name()
        ))),
    }
}

fn apply_builtin(fun: fn(Vec<Sexp>) -> Result<Sexp>, args: Vec<Sexp>) -> Result<Sexp> {
    fun(args)
}
