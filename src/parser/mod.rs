#![allow(dead_code)]
use itertools::Itertools;
use pest::iterators::Pair;
use pest_derive::Parser;

use thiserror::Error;

use std::num::ParseIntError;

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct LispParser;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum Error {
    #[error("failed to parse int")]
    InvalidInt(#[from] ParseIntError),

    #[error("expected expr got: {0}")]
    MissingExpr(String),

    #[error("expected int got: {0}")]
    MissingInt(String),

    #[error("unexpected token: {0:?}")]
    UnexpectedToken(Rule),
}

#[derive(Debug, Clone)]
pub enum Ast {
    Int(i64),
    Str(String),
    Symbol(String),
    SExpr(Vec<Ast>),
    QExpr(Vec<Ast>),
    Binding { symbol: String, expr: Box<Ast> },
}

pub fn read(parsed: Pair<Rule>) -> Result<Ast, Error> {
    fn into_inner_vec(pair: Pair<Rule>) -> Result<Vec<Ast>, Error> {
        pair.into_inner()
            .filter(|p| !matches!(p.as_rule(), Rule::EOI | Rule::COMMENT))
            .map(read)
            .collect::<Result<Vec<_>, Error>>()
    }

    let span = parsed.clone().as_span().as_str();

    match parsed.as_rule() {
        Rule::sexpr => Ok(Ast::SExpr(into_inner_vec(parsed)?)),
        Rule::qexpr => Ok(Ast::QExpr(into_inner_vec(parsed)?)),
        Rule::binding => {
            let (symbol, expr) = parsed
                .into_inner()
                .next_tuple()
                .ok_or(Error::MissingExpr(span.to_string()))?;
            Ok(Ast::Binding {
                symbol: symbol.as_span().as_str().to_string(),
                expr: Box::new(read(expr)?),
            })
        }
        Rule::expr => {
            let inner = parsed
                .into_inner()
                .next()
                .ok_or(Error::MissingExpr(span.to_string()))?;
            read(inner)
        }
        Rule::number => Ok(Ast::Int(span.parse::<i64>()?)),
        //string without outer quotes
        Rule::string => Ok(Ast::Str(span[1..span.len() - 1].to_string())),
        Rule::symbol => Ok(Ast::Symbol(span.to_string())),
        _ => Err(Error::UnexpectedToken(parsed.as_rule())),
    }
}
