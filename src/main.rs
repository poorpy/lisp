mod eval;
mod parser;

use eval::{Env, Expr};
use pest::Parser;

fn main() {
    let input = std::fs::read_to_string("input.lisp").expect("cannot read file");
    let pairs =
        parser::LispParser::parse(parser::Rule::lisp, &input).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        if let Ok(ast) = parser::read(pair) {
            println!("{:?}", eval::eval(Expr::from(ast), &mut Env::default()));
        }
    }
}
