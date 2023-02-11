use std::collections::HashMap;

use crate::eval::{Error, Expr, Result};

#[allow(dead_code)]
pub struct Env<'a> {
    data: HashMap<String, Expr>,
    outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            outer: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_outer(outer: &'a Env<'a>) -> Self {
        Self {
            data: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn insert(&mut self, symbol: String, expr: Expr) {
        self.data.insert(symbol, expr);
    }

    pub fn get(&self, symbol: &str) -> Option<Expr> {
        self.data.get(symbol).cloned()
    }
}

impl<'a> Default for Env<'a> {
    fn default() -> Self {
        let mut env = Self::new();

        env.data.insert(
            "add".to_string(),
            Expr::Func {
                name: "add".to_string(),
                fun: add,
            },
        );

        env
    }
}

pub fn add(args: Vec<Expr>) -> Result<Expr> {
    let mut result = 0;
    for arg in args {
        if let Expr::Int(i) = arg {
            result += i;
        } else {
            return Err(Error::InvalidType {
                expected: String::new(),
                actual: String::new(),
            });
        }
    }

    Ok(Expr::Int(result))
}
