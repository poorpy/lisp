use super::{eval, RuntimeError};
use crate::env::Env;
use crate::parser::{Atom, Sexp};

#[test]
fn non_symbol_atom_evals_to_itself() {
    let mut env = Env::new(None);
    let sexp = Sexp::Atom(Atom::T);
    assert_eq!(eval(sexp, &mut env), Ok(Sexp::Atom(Atom::T)))
}

#[test]
fn empty_list_evals_to_nil() {
    let mut env = Env::new(None);
    let sexp = Sexp::List(Vec::new());
    assert_eq!(eval(sexp, &mut env), Ok(Sexp::Atom(Atom::Nil)))
}

#[test]
fn builtin_is_applied() {
    let mut env = Env::default();
    let sexp = Sexp::from(vec![
        Sexp::Atom(Atom::Symbol("atom?".to_string())),
        Sexp::Atom(Atom::Nil),
    ]);
    assert_eq!(eval(sexp, &mut env), Ok(Sexp::Atom(Atom::T)))
}

#[test]
fn eval_fails_on_non_func_element_of_list() {
    let mut env = Env::default();
    let sexp = Sexp::from(vec![Sexp::Atom(Atom::Nil)]);
    assert_eq!(
        eval(sexp, &mut env),
        Err(RuntimeError::WrongArgumentKind(
            "eval list, expected function, got: atom".to_string()
        ))
    )
}
