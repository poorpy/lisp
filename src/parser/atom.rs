#![allow(dead_code)]

use std::collections::HashSet;

use nom::IResult;

use crate::core;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::combinator::map;
use nom::Err::Error;

use super::ParserError;

fn boolean(input: &str) -> IResult<&str, core::Atom> {
    alt((
        map(tag("false"), |_| core::Atom::Boolean(false)),
        map(tag("true"), |_| core::Atom::Boolean(true)),
    ))(input)
}

fn symbol(input: &str) -> IResult<&str, core::Atom, ParserError<&str>> {
    let (rest, candidate): (&str, &str) = take_till(|c: char| c.is_whitespace())(input)?;

    let illegal = candidate
        .chars()
        .collect::<HashSet<char>>()
        .intersection(&HashSet::from_iter("(){},;.:'\"".chars()))
        .cloned()
        .collect::<HashSet<char>>();

    if !illegal.is_empty() {
        return Err(Error(ParserError::SymbolContainsInvalid {
            symbol: candidate.to_owned(),
            illegal,
        }));
    }

    Ok((rest, core::Atom::Symbol(candidate.to_owned())))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{core, parser::ParserError};

    use rstest::rstest;

    #[rstest]
    #[case("true", core::Atom::Boolean(true))]
    #[case("false", core::Atom::Boolean(false))]
    fn test_parse_boolean(#[case] input: &str, #[case] expected: core::Atom) {
        let result = super::boolean(input);
        assert_eq!(result.unwrap().1, expected)
    }

    #[test]
    fn test_parse_symbol() {
        let result = super::symbol("my_awesome-symbol123");
        assert_eq!(
            result.unwrap().1,
            core::Atom::Symbol("my_awesome-symbol123".to_owned())
        )
    }

    #[test]
    fn test_parse_symbol_fails_on_forbidden() {
        let result = super::symbol("my(){},;.:'\"_awesome-symbol123");
        assert_eq!(
            result.unwrap_err(),
            nom::Err::Error(ParserError::SymbolContainsInvalid {
                symbol: "my(){},;.:'\"_awesome-symbol123".to_owned(),
                illegal: HashSet::from_iter("(){},;.:'\"".chars()),
            }),
        )
    }
}
