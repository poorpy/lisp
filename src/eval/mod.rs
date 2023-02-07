#![allow(dead_code)]
use super::parser::Ast;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum Error {
    #[error("something went wrong")]
    Unit,
}

pub fn eval(ast: Ast) -> Result<Ast, Error> {
    match ast {
        Ast::Int(_) | Ast::Str(_) => Ok(ast),
        _ => unimplemented!(),
    }
}
