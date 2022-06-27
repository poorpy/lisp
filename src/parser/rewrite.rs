use std::collections::VecDeque;

use super::{Atom, Sexp};

pub fn rewrite_quotes(ast: VecDeque<Sexp>) -> VecDeque<Sexp> {
    let mut new_ast = VecDeque::new();
    let mut iter = ast.into_iter();
    while let Some(expr) = iter.next() {
        match expr {
            Sexp::Atom(Atom::Symbol(symbol)) if symbol == "'" => {
                if let Some(next) = iter.next() {
                    match next {
                        Sexp::List(list) => {
                            new_ast.push_back(Sexp::from(vec![
                                Sexp::Atom(Atom::Symbol("quote".to_string())),
                                Sexp::List(rewrite_quotes(list)),
                            ]));
                        }
                        _ => {
                            new_ast.push_back(Sexp::from(vec![
                                Sexp::Atom(Atom::Symbol("quote".to_string())),
                                next,
                            ]));
                        }
                    }
                } else {
                    new_ast.push_back(Sexp::Atom(Atom::Symbol("quote".to_string())))
                }
            }
            Sexp::List(list) => new_ast.push_back(Sexp::List(rewrite_quotes(list))),
            _ => new_ast.push_back(expr),
        }
    }
    new_ast
}
