use super::{Atom, Sexp};

pub fn parse(token: String) -> Sexp {
    if let "t" | "T" | "true"  = token.as_str() {
        return Sexp::Atom(Atom::T);
    }

    if let "nil" | "()" | "false" = token.as_str() {
        return Sexp::Atom(Atom::Nil);
    }
    
    if let Ok(f) = token.parse::<f64>() {
        return Sexp::Atom(Atom::Number(f));
    }

    Sexp::Atom(Atom::Symbol(token))
}
