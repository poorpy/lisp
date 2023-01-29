use std::collections::HashSet;

use nom::error::{ErrorKind, ParseError};
use thiserror::Error;

mod atom;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum ParserError<I> {
    #[error("symbol {symbol} contains illegal characters: {illegal:?}")]
    SymbolContainsInvalid {
        symbol: String,
        illegal: HashSet<char>,
    },
    #[error("internal parser failed with error {1:?} for input {0:?}")]
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for ParserError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ParserError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}
