use std::{fs::File, io::{BufReader, BufRead}};

mod lexer;

use lexer::tokenize;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("Unable to open file"));
    let lines = file.lines();
    println!("{:?}", tokenize(lines));
}
