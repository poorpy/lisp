#![allow(dead_code)]
#[cfg(test)]
mod tests;

use std::{char, io, iter::Enumerate, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Symbol(String),
    String(String),
    LParen,
    RParen,
}

const COMMENT: char = '#';

pub fn tokenize<I>(source: I) -> Result<Vec<Token>>
where
    I: Iterator<Item = std::result::Result<String, io::Error>>,
{
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line) in source.enumerate() {
        if let Err(err) = line {
            return Err(LexerError {
                column: 0,
                line: line_number,
                message: format!("Error reading line {}: {}", line_number, err),
            });
        }

        let line = line.unwrap();
        let mut chars = line.chars().enumerate();

        while let Some((_, c)) = chars.next() {
            match c {
                COMMENT => break,
                c if c.is_whitespace() => continue,
                '\"' => tokens.push(tokenize_string(&mut chars, line_number)?),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                c => tokens
                    .extend(tokenize_symbol(c.to_string(), &mut chars, line_number)?.into_iter()),
            }
        }
    }

    Ok(tokens)
}

fn tokenize_string(chars: &mut Enumerate<Chars>, line: usize) -> Result<Token> {
    let mut string = String::new();
    let mut column: usize = 0;
    for (col, c) in chars {
        column = col;
        if c == '\"' {
            return Ok(Token::String(string));
        }
        string.push(c);
    }
    Err(LexerError {
        column,
        line,
        message: String::from("Error lexing string: missing closing \""),
    })
}

fn tokenize_symbol(
    mut symbol: String,
    chars: &mut Enumerate<Chars>,
    line: usize,
) -> Result<Vec<Token>> {
    fn is_forbidden(c: char) -> bool {
        matches!(c, '\"' | '\'')
    }

    fn is_terminal(c: char) -> bool {
        match c {
            c if c.is_whitespace() => true,
            '(' | ')' => true,
            _ => false,
        }
    }

    for (column, c) in chars.by_ref() {
        if is_forbidden(c) {
            return Err(LexerError {
                line,
                column,
                message: format!("Error lexing symbol: forbidden character: {}", c),
            });
        }

        if is_terminal(c) {
            if matches!(c, '(' | ')') {
                let terminal = if c == '(' {
                    Token::LParen
                } else {
                    Token::RParen
                };
                return Ok(vec![Token::Symbol(symbol), terminal]);
            }

            return Ok(vec![Token::Symbol(symbol)]);
        }

        symbol.push(c)
    }

    Ok(vec![Token::Symbol(symbol)])
}
