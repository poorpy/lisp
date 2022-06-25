use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

mod env;
mod eval;
mod lexer;
mod parser;

use env::Env;
use eval::eval;
use lexer::tokenize;
use parser::read_from_tokens;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("Unable to open file"));
    let lines = file.lines();
    let tokens = tokenize(lines);

    if let Err(err) = tokens {
        println!("{:?}", err);
        exit(0)
    }

    let expressions = read_from_tokens(tokens.unwrap());

    if let Err(err) = expressions {
        println!("{:?}", err);
        exit(0)
    }

    let expressions = expressions.unwrap();
    let mut env = Env::default();
    for expr in expressions {
        println!("{:?}", eval(expr, &mut env));
    }
}
