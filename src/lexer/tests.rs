use super::{tokenize, LexerError, Token};
use std::io::BufRead;

#[test]
fn tokenizer_fails_on_missing_quote() {
    let code = "\"invalid string".as_bytes().lines();
    let result = tokenize(code);
    assert_eq!(
        result,
        Err(LexerError {
            line: 0,
            column: 14,
            message: String::from("error lexing string: missing closing \"")
        }),
    )
}

#[test]
fn tokenizer_returns_simple_sequence() {
    let code = "(+ 1 2 3)".as_bytes().lines();
    let result = tokenize(code);
    assert_eq!(
        result,
        Ok(vec![
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Symbol("1".to_string()),
            Token::Symbol("2".to_string()),
            Token::Symbol("3".to_string()),
            Token::RParen,
        ])
    )
}
