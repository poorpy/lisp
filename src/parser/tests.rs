use crate::lexer::Token;

use super::{read_from_tokens, Atom, ParserError, Sexp};

#[test]
fn parser_fails_on_unexpected_rparen() {
    let tokens = vec![Token::RParen];
    let result = read_from_tokens(tokens);
    assert_eq!(
        result,
        Err(ParserError {
            message: String::from("unexpected closing paren")
        })
    )
}

#[test]
fn parser_parses_atom() {
    let tokens = vec![
        // empty list, nil and false should parse as nil
        Token::LParen,
        Token::RParen,
        Token::Symbol(String::from("nil")),
        Token::Symbol(String::from("false")),
        Token::Symbol(String::from("t")),
        Token::Symbol(String::from("true")),
        Token::Symbol(String::from("cons")),
        Token::String(String::from("text")),
        Token::Symbol(String::from("1.0")),
        Token::Symbol(String::from("1")),
    ];
    let result = read_from_tokens(tokens);
    assert_eq!(
        result,
        Ok(vec![
            Sexp::Atom(Atom::Nil),
            Sexp::Atom(Atom::Nil),
            Sexp::Atom(Atom::Nil),
            Sexp::Atom(Atom::T),
            Sexp::Atom(Atom::T),
            Sexp::Atom(Atom::Symbol(String::from("cons"))),
            Sexp::Atom(Atom::String(String::from("text"))),
            Sexp::Atom(Atom::Number(1.0)),
            Sexp::Atom(Atom::Number(1.0))
        ])
    )
}

#[test]
fn parser_parses_list() {
    let tokens = vec![
        Token::LParen,
        Token::Symbol(String::from("nil")),
        Token::Symbol(String::from("nil")),
        Token::Symbol(String::from("nil")),
        Token::RParen,
    ];
    let result = read_from_tokens(tokens);
    assert_eq!(
        result,
        Ok(vec![Sexp::List(vec![
            Sexp::Atom(Atom::Nil),
            Sexp::Atom(Atom::Nil),
            Sexp::Atom(Atom::Nil),
        ])])
    )
}
