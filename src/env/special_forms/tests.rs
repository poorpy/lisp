use std::collections::VecDeque;

use crate::eval::Result;
use crate::eval::RuntimeError;
use crate::parser::{Atom, Sexp};

use super::{append, prepend};
use super::{car, cdr, list};

use rstest::rstest;

#[test]
fn can_create_list() {
    let args = VecDeque::from(vec![Sexp::Atom(Atom::T)]);
    assert_eq!(list(args), Ok(Sexp::from(vec![Sexp::Atom(Atom::T)])))
}

#[test]
fn list_create_fails_with_no_args() {
    let args = VecDeque::new();
    assert_eq!(list(args), Err(RuntimeError::WrongArgumentArity(0)))
}

#[rstest]
#[case(Sexp::Atom(Atom::Nil), Ok(Sexp::Atom(Atom::Nil)))]
#[case(Sexp::from(vec![]), Ok(Sexp::Atom(Atom::Nil)))]
#[case(Sexp::from(vec![Sexp::Atom(Atom::T)]), Ok(Sexp::Atom(Atom::T)))]
fn car_returns_head_of_a_list(#[case] input: Sexp, #[case] expected: Result<Sexp>) {
    let args = VecDeque::from(vec![input]);
    assert_eq!(car(args), expected)
}

#[rstest]
#[case(Sexp::Atom(Atom::Nil), Ok(Sexp::Atom(Atom::Nil)))]
#[case(Sexp::from(vec![]), Ok(Sexp::Atom(Atom::Nil)))]
#[case(Sexp::from(vec![Sexp::Atom(Atom::T)]), Ok(Sexp::from(vec![])))]
#[case(Sexp::from(vec![Sexp::Atom(Atom::T), Sexp::Atom(Atom::T)]), Ok(Sexp::from(vec![Sexp::Atom(Atom::T)])))]
fn cdr_returns_tail_of_a_list(#[case] input: Sexp, #[case] expected: Result<Sexp>) {
    let args = VecDeque::from(vec![input]);
    assert_eq!(cdr(args), expected)
}

#[rstest]
#[case(
    VecDeque::from(vec![Sexp::Atom(Atom::Nil), Sexp::Atom(Atom::T)]),
    Ok(Sexp::from(vec![Sexp::Atom(Atom::T)]))
)]
#[case(
    VecDeque::from(vec![Sexp::from(vec![Sexp::Atom(Atom::Nil)]), Sexp::Atom(Atom::T)]),
    Ok(Sexp::from(vec![Sexp::Atom(Atom::Nil), Sexp::Atom(Atom::T)]))
)]
fn append_pushes_arg_to_back(#[case] input: VecDeque<Sexp>, #[case] expected: Result<Sexp>) {
    assert_eq!(append(input), expected)
}

#[rstest]
#[case(
    VecDeque::from(vec![Sexp::Atom(Atom::Nil), Sexp::Atom(Atom::T)]),
    Ok(Sexp::from(vec![Sexp::Atom(Atom::T)]))
)]
#[case(
    VecDeque::from(vec![Sexp::from(vec![Sexp::Atom(Atom::Nil)]), Sexp::Atom(Atom::T)]),
    Ok(Sexp::from(vec![Sexp::Atom(Atom::T), Sexp::Atom(Atom::Nil)]))
)]
fn prepend_pushes_arg_to_front(#[case] input: VecDeque<Sexp>, #[case] expected: Result<Sexp>) {
    assert_eq!(prepend(input), expected)
}
