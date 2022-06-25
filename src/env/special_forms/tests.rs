use crate::eval::RuntimeError;
use crate::parser::{Atom, Sexp};


use super::list;

#[test]
fn can_create_list() {
    let args = Sexp::from(vec![Sexp::Atom(Atom::T)]);
    assert_eq!(list(args), Ok(Sexp::List(vec![Sexp::Atom(Atom::T)])))
}

#[test]
fn list_create_fails_with_no_args() {
    let args = Sexp::from(vec![]);
    assert_eq!(list(args), Err(RuntimeError::WrongArgumentArity(0)))
}
