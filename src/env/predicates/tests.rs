use crate::eval::Result;
use crate::parser::{Atom, Sexp};

use super::{is_atom, is_function, is_list, is_string, is_symbol, is_number};

use rstest::rstest;

#[rstest]
#[case(Sexp::Atom(Atom::T), Ok(Sexp::Atom(Atom::T)))]
#[case(Sexp::List(vec![]), Ok(Sexp::Atom(Atom::Nil)))]
fn is_atom_works_correctly(#[case] input: Sexp, #[case] expected: Result<Sexp>) {
    let args = Sexp::from(vec![input]);
    assert_eq!(is_atom(args), expected)
}

#[rstest]
#[case(Sexp::Atom(Atom::T), Ok(Sexp::Atom(Atom::Nil)))]
#[case(Sexp::List(vec![]), Ok(Sexp::Atom(Atom::T)))]
#[case(Sexp::Atom(Atom::Nil), Ok(Sexp::Atom(Atom::T)))]
fn is_list_works_correctly(#[case] input: Sexp, #[case] expected: Result<Sexp>) {
    let args = Sexp::from(vec![input]);
    assert_eq!(is_list(args), expected)
}

#[rstest]
#[case(Sexp::Atom(Atom::T), Ok(Sexp::Atom(Atom::Nil)))]
#[case(Sexp::List(vec![]), Ok(Sexp::Atom(Atom::Nil)))]
#[case(Sexp::Func { fun: is_function, name: "" }, Ok(Sexp::Atom(Atom::T)))]
fn is_function_works_correctly(#[case] input: Sexp, #[case] expected: Result<Sexp>) {
    let args = Sexp::from(vec![input]);
    assert_eq!(is_function(args), expected)
}

#[rstest]
#[case(Sexp::Atom(Atom::String("string".to_string())), Ok(Sexp::Atom(Atom::T)))]
#[case(Sexp::List(vec![]), Ok(Sexp::Atom(Atom::Nil)))]
#[case(Sexp::Func { fun: is_function, name: "" }, Ok(Sexp::Atom(Atom::Nil)))]
fn is_string_works_correctly(#[case] input: Sexp, #[case] expected: Result<Sexp>) {
    let args = Sexp::from(vec![input]);
    assert_eq!(is_string(args), expected)
}

#[rstest]
#[case(Sexp::Atom(Atom::Symbol("symbol".to_string())), Ok(Sexp::Atom(Atom::T)))]
#[case(Sexp::List(vec![]), Ok(Sexp::Atom(Atom::Nil)))]
fn is_symbol_works_correctly(#[case] input: Sexp, #[case] expected: Result<Sexp>) {
    let args = Sexp::from(vec![input]);
    assert_eq!(is_symbol(args), expected)
}

#[rstest]
#[case(Sexp::Atom(Atom::Number(1.0)), Ok(Sexp::Atom(Atom::T)))]
#[case(Sexp::List(vec![]), Ok(Sexp::Atom(Atom::Nil)))]
fn is_number_works_correctly(#[case] input: Sexp, #[case] expected: Result<Sexp>) {
    let args = Sexp::from(vec![input]);
    assert_eq!(is_number(args), expected)
}
