mod predicates;
mod special_forms;

use std::collections::HashMap;

use crate::env::{
    predicates::{is_atom, is_function, is_list, is_number, is_string, is_symbol},
    special_forms::{car, cdr, list},
};

use super::parser::Sexp;

macro_rules! add_func_to_env {
    ($ name : expr, $ func : expr, $ env : expr) => {
        $env.data.insert(
            $name.to_string(),
            Sexp::Func {
                fun: $func,
                name: $name,
            },
        )
    };
}

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

        // predicates
        add_func_to_env!("atom?", is_atom, default);
        add_func_to_env!("list?", is_list, default);
        add_func_to_env!("function?", is_function, default);
        add_func_to_env!("string?", is_string, default);
        add_func_to_env!("symbol?", is_symbol, default);
        add_func_to_env!("number?", is_number, default);

        // list manipulation
        add_func_to_env!("list", list, default);
        add_func_to_env!("car", car, default);
        add_func_to_env!("cdr", cdr, default);

        default
    }
}
