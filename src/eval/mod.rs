#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

use std::fmt;

use thiserror::Error;

use crate::{
    env::{self, Env},
    parser::Ast,
};

#[derive(Debug, Error, PartialEq, Clone)]
pub enum Error {
    #[error("something went wrong")]
    Unit,

    #[error("type mismatch expected: {expected:?} got: {actual:?}")]
    InvalidType {
        expected: &'static str,
        actual: &'static str,
    },

    #[error("function {name} expected {expected} arguments instead got {actual}")]
    BadArity {
        name: String,
        expected: usize,
        actual: usize,
    },

    #[error("tried to divide by zero")]
    DivideByZero,

    #[error("tired to evaluate undefined symbol: {symbol} ")]
    Undefined { symbol: String },
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
    Func {
        name: &'static str,
        fun: Builtin,
    },
    Binding {
        symbol: String,
        expr: Box<Expr>,
    },
    Lambda {
        formals: Vec<String>,
        body: Box<Expr>,
    },
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
            Self::Lambda { .. } => "lambda",
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
            Ast::Lambda { formals, body } => Self::Lambda {
                formals,
                body: Box::new(Self::from(*body)),
            },
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn space_separated<T: ToString>(vec: &Vec<T>) -> String {
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
            Self::Func { name, .. } => write!(f, "{name}"),
            Self::Binding { symbol, expr } => write!(f, "( let {symbol} {expr} )"),
            Self::Lambda { formals, body } => {
                let args = space_separated(formals);
                write!(f, "( lambda ( {args} ) {body} )")
            }
        }
    }
}

pub fn eval(expr: Expr, env: &mut env::Env) -> Result<Expr> {
    match expr {
        Expr::Int(_) | Expr::Str(_) | Expr::Func { .. } | Expr::Lambda { .. } => Ok(expr),
        Expr::Symbol(s) => Ok(env.get(&s).ok_or(Error::Undefined { symbol: s })?),
        Expr::QExpr(vec) => Ok(Expr::SExpr(vec)),
        Expr::SExpr(vec) => {
            let evaluated = vec
                .into_iter()
                .map(|e| eval(e, env))
                .collect::<Result<Vec<Expr>>>()?;

            if let Expr::Func { fun, .. } = evaluated[0] {
                return apply(fun, evaluated[1..].to_vec());
            }

            if let Expr::Lambda { formals, body } = &evaluated[0] {
                if evaluated[1..].len() != formals.len() {
                    return Err(Error::BadArity {
                        name: "lambda".to_string(),
                        expected: formals.len(),
                        actual: evaluated[1..].len(),
                    });
                }

                let mut lambda_env = Env::with_outer(env);
                for (formal, expr) in formals.iter().zip(evaluated[1..].iter()) {
                    lambda_env.insert(formal.clone(), expr.clone())
                }

                return eval(*(*body).clone(), &mut lambda_env);
            }

            Err(Error::InvalidType {
                expected: "builtin or lambda",
                actual: evaluated[0].typename(),
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
