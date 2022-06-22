use std::{fs::File, io::{BufReader, BufRead}};

mod lexer;
mod parser;
mod eval;
mod env;

use lexer::tokenize;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("Unable to open file"));
    let lines = file.lines();
    println!("{:?}", tokenize(lines));
}
