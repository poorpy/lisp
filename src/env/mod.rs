use std::collections::HashMap;

use super::parser::Sexp;

#[derive(Debug, Clone, PartialEq)]
pub struct LookupError {
    pub message: String
}

pub struct Env<'a> {
    pub data: HashMap<String, Sexp>,
    pub outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn search(&self, _symbol: String) -> std::result::Result<Sexp, LookupError> {
        unimplemented!()
    }

    pub fn new(outer: Option<&'a Env<'a>>) -> Env<'a> {
        Env { data: HashMap::new(), outer }
    }
}
