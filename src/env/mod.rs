mod mathop;

use std::collections::HashMap;

use crate::eval::Expr;

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

macro_rules! add_func_to_env {
    ($ name : expr, $ func : expr, $ env : expr) => {
        $env.data.insert(
            $name.to_string(),
            Expr::Func {
                fun: $func,
                name: $name,
            },
        )
    };
}

impl<'a> Default for Env<'a> {
    fn default() -> Self {
        let mut env = Self::new();

        add_func_to_env!("add", mathop::add, env);

        env
    }
}
