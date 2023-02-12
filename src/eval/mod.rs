#![allow(dead_code)]
mod builtin;

use std::fmt;

use thiserror::Error;

use crate::{env, parser::Ast};

#[derive(Debug, Error, PartialEq, Clone)]
pub enum Error {
    #[error("something went wrong")]
    Unit,

    #[error("type mismatch expected: {expected:?} got: {actual:?}")]
    InvalidType { expected: String, actual: String },
}

pub type Result<T> = std::result::Result<T, Error>;
pub type Builtin = fn(Vec<Expr>) -> Result<Expr>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Int(i64),
    Str(String),
    Symbol(String),
    SExpr(Vec<Expr>),
    QExpr(Vec<Expr>),
    Func { name: &'static str, fun: Builtin },
    Binding { symbol: String, expr: Box<Expr> },
}

impl Expr {
    pub fn typename(&self) -> &'static str {
        match self {
            Self::Int(_) => "number: int",
            Self::Str(_) => "string",
            Self::Symbol(_) => "symbol",
            Self::SExpr(_) => "s-expression",
            Self::QExpr(_) => "q-expression",
            Self::Func { .. } => "builtin",
            Self::Binding { .. } => "binding",
        }
    }
}

impl From<Ast> for Expr {
    fn from(value: Ast) -> Self {
        match value {
            Ast::Int(i) => Self::Int(i),
            Ast::Str(s) => Self::Str(s),
            Ast::Symbol(s) => Self::Symbol(s),
            Ast::SExpr(v) => Self::SExpr(v.into_iter().map(Self::from).collect()),
            Ast::QExpr(v) => Self::QExpr(v.into_iter().map(Self::from).collect()),
            Ast::Binding { symbol, expr } => Self::Binding {
                symbol,
                expr: Box::new(Self::from(*expr)),
            },
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn space_separated(vec: &Vec<Expr>) -> String {
            let mut space_separated = String::new();
            for expr in vec {
                space_separated.push_str(&expr.to_string());
                space_separated.push(' ');
            }
            space_separated.trim_end().to_string()
        }

        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Str(s) => write!(f, "\"{s}\""),
            Self::Symbol(s) => write!(f, "{s}"),
            Self::SExpr(vec) => {
                let repr = space_separated(vec);
                write!(f, "( {repr} )")
            }
            Self::QExpr(vec) => {
                let repr = space_separated(vec);
                write!(f, "{{ {repr} }}")
            }
            Self::Func { name, .. } => write!(f, "builtin: {name}"),
            Self::Binding { symbol, expr } => write!(f, "( let {symbol} {expr} )"),
        }
    }
}

pub fn eval(expr: Expr, env: &mut env::Env) -> Result<Expr> {
    match expr {
        Expr::Int(_) | Expr::Str(_) | Expr::Func { .. } => Ok(expr),
        Expr::Symbol(s) => Ok(env.get(&s).unwrap()),
        Expr::QExpr(vec) => Ok(Expr::SExpr(vec)),
        Expr::SExpr(vec) => {
            let evaluated = vec
                .into_iter()
                .map(|e| eval(e, env))
                .collect::<Result<Vec<Expr>>>()?;

            if let Expr::Func { fun, .. } = evaluated[0] {
                return apply(fun, evaluated[1..].to_vec());
            }

            Err(Error::InvalidType {
                expected: "builtin".to_string(),
                actual: evaluated[0].typename().to_string(),
            })
        }
        Expr::Binding { symbol, expr } => {
            env.insert(symbol, (*expr).clone());
            Ok(*expr)
        }
    }
}

pub fn apply(fun: fn(Vec<Expr>) -> Result<Expr>, args: Vec<Expr>) -> Result<Expr> {
    fun(args)
}
