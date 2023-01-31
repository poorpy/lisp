#![allow(dead_code)]

use std::collections::HashSet;

use nom::branch::alt;
use nom::{
    bytes::complete::{escaped, tag, take_till},
    character::complete::{none_of, one_of},
    combinator::map,
    number::complete::double,
    sequence::delimited,
    Err::Failure,
    IResult,
};

use crate::core::Atom;

use super::ParserError;

fn atom(input: &str) -> IResult<&str, Atom, ParserError<&str>> {
    alt((boolean, symbol, string, number))(input)
}

fn boolean(input: &str) -> IResult<&str, Atom, ParserError<&str>> {
    alt((
        map(tag("false"), |_| Atom::Boolean(false)),
        map(tag("true"), |_| Atom::Boolean(true)),
    ))(input)
}

fn symbol(input: &str) -> IResult<&str, Atom, ParserError<&str>> {
    let (rest, candidate): (&str, &str) = take_till(|c: char| c.is_whitespace())(input)?;

    let illegal = candidate
        .chars()
        .collect::<HashSet<char>>()
        .intersection(&HashSet::from_iter("(){},;.:'\"".chars()))
        .copied()
        .collect::<HashSet<char>>();

    if !illegal.is_empty() {
        return Err(Failure(ParserError::SymbolContainsInvalid {
            symbol: candidate.to_owned(),
            illegal,
        }));
    }

    Ok((rest, Atom::Symbol(candidate.to_owned())))
}

fn string(input: &str) -> IResult<&str, Atom, ParserError<&str>> {
    map(
        delimited(
            tag("\""),
            escaped(none_of("\"\n\\"), '\\', one_of(r#""n\"#)),
            tag("\""),
        ),
        |s: &str| Atom::String(s.to_owned()),
    )(input)
}

fn number(input: &str) -> IResult<&str, Atom, ParserError<&str>> {
    map(double, Atom::Number)(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{core::Atom, parser::ParserError};

    use rstest::rstest;

    #[rstest]
    #[case("true", Atom::Boolean(true))]
    #[case("false", Atom::Boolean(false))]
    fn test_parse_boolean(#[case] input: &str, #[case] expected: Atom) {
        let result = super::boolean(input);
        assert_eq!(result.unwrap().1, expected)
    }

    #[test]
    fn test_parse_symbol() {
        let result = super::symbol("my_awesome-symbol123");
        assert_eq!(
            result.unwrap().1,
            Atom::Symbol("my_awesome-symbol123".to_owned())
        )
    }

    #[test]
    fn test_parse_symbol_fails_on_forbidden() {
        let result = super::symbol("my(){},;.:'\"_awesome-symbol123");
        assert_eq!(
            result.unwrap_err(),
            nom::Err::Failure(ParserError::SymbolContainsInvalid {
                symbol: "my(){},;.:'\"_awesome-symbol123".to_owned(),
                illegal: HashSet::from_iter("(){},;.:'\"".chars()),
            }),
        )
    }

    #[test]
    fn test_parse_string() {
        let result = super::string(r#""my cool and (){}\n\\\"\"string""#);

        assert_eq!(
            result.unwrap().1,
            Atom::String(r#"my cool and (){}\n\\\"\"string"#.to_owned()),
        )
    }
}
