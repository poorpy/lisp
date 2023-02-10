#![allow(dead_code)]
mod builtin;

use std::collections::HashMap;

use thiserror::Error;

use crate::parser::Ast;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum Error {
    #[error("something went wrong")]
    Unit,

    #[error("type mismatch expected: {expected:?} got: {actual:?}")]
    InvalidType { expected: String, actual: String },
}

type Builtin = fn(Vec<Expr>) -> Result<Expr>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Int(i64),
    Str(String),
    Symbol(String),
    SExpr(Vec<Expr>),
    QExpr(Vec<Expr>),
    // TODO: impl PartialEq with builtins accepting slices
    Func { name: String, fun: Builtin },
}

impl From<Ast> for Expr {
    fn from(value: Ast) -> Self {
        match value {
            Ast::Int(i) => Self::Int(i),
            Ast::Str(s) => Self::Str(s),
            Ast::Symbol(s) => Self::Symbol(s),
            Ast::SExpr(v) => Self::SExpr(v.into_iter().map(Self::from).collect()),
            Ast::QExpr(v) => Self::QExpr(v.into_iter().map(Self::from).collect()),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

pub struct Env {
    pub data: HashMap<String, Expr>,
}

impl Default for Env {
    fn default() -> Self {
        let mut env = Self {
            data: HashMap::new(),
        };

        env.data.insert(
            "add".to_string(),
            Expr::Func {
                name: "add".to_string(),
                fun: builtin::add,
            },
        );

        env
    }
}

pub fn eval(expr: Expr, env: &mut Env) -> Result<Expr> {
    match expr {
        Expr::Int(_) | Expr::Str(_) => Ok(expr),
        Expr::Symbol(s) => Ok(env.data.get(&s).unwrap().clone()),
        Expr::Func { .. } => Ok(expr),
        Expr::SExpr(vec) => {
            let evaluated = vec
                .into_iter()
                .map(|e| eval(e, env))
                .collect::<Result<Vec<Expr>>>()?;
            if let Expr::Func { fun, .. } = evaluated[0] {
                return apply(fun, evaluated[1..].to_vec());
            }

            Err(Error::InvalidType {
                expected: "function".to_string(),
                actual: "whatever".to_string(),
            })
        }
        _ => unimplemented!(),
    }
}

pub fn apply(fun: fn(Vec<Expr>) -> Result<Expr>, args: Vec<Expr>) -> Result<Expr> {
    fun(args)
}
