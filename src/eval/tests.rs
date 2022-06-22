use crate::env::Env;

use crate::parser::{Atom, Sexp};

use super::eval;

#[test]
fn non_symbol_atom_evals_to_itself() {
    let mut env = Env::new(None);
    let sexp = Sexp::Atom(Atom::T);
    assert_eq!(eval(sexp, &mut env), Ok(Sexp::Atom(Atom::T)))
}
