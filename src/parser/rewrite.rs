use super::{Atom, Sexp};

pub fn rewrite_quotes(ast: Vec<Sexp>) -> Vec<Sexp> {
    let mut new_ast = Vec::new();
    let mut iter = ast.into_iter();
    while let Some(expr) = iter.next() {
        match expr {
            Sexp::Atom(Atom::Symbol(symbol)) if symbol == "'" => {
                if let Some(next) = iter.next() {
                    match next {
                        Sexp::List(list) => {
                            new_ast.push(Sexp::List(vec![
                                Sexp::Atom(Atom::Symbol("quote".to_string())),
                                Sexp::List(rewrite_quotes(list)),
                            ]));
                        }
                        _ => {
                            new_ast.push(Sexp::List(vec![
                                Sexp::Atom(Atom::Symbol("quote".to_string())),
                                next,
                            ]));
                        }
                    }
                }
            }
            Sexp::List(list) => new_ast.push(Sexp::List(rewrite_quotes(list))),
            _ => new_ast.push(expr),
        }
    }
    new_ast
}
