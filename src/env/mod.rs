mod predicates;

use std::collections::HashMap;

use self::predicates::is_atom;

use super::parser::Sexp;

#[derive(Debug, Clone, PartialEq)]
pub struct LookupError {
    pub message: String,
}

pub struct Env<'a> {
    pub data: HashMap<String, Sexp>,
    pub outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn search(&self, symbol: String) -> std::result::Result<Sexp, LookupError> {
        match self.data.get(&symbol) {
            Some(sexp) => Ok(sexp.clone()),
            None => match self.outer {
                None => Err(LookupError { message: symbol }),
                Some(outer) => outer.search(symbol),
            },
        }
    }

    pub fn new(outer: Option<&'a Env<'a>>) -> Env<'a> {
        Env {
            data: HashMap::new(),
            outer,
        }
    }
}

impl<'a> Default for Env<'a> {
    fn default() -> Self {
        let mut default = Env::new(None);

        default.data.insert(
            "atom?".to_string(),
            Sexp::Func {
                fun: is_atom,
                name: "atom?",
            },
        );

        default
    }
}
