mod env;
mod eval;
mod parser;

use env::Env;
use eval::Expr;
use parser::Rule;
use pest::Parser;

fn main() {
    let input = std::fs::read_to_string("input.lisp").expect("cannot read file");
    let pairs =
        parser::LispParser::parse(parser::Rule::lisp, &input).unwrap_or_else(|e| panic!("{}", e));

    let mut env = Env::default();

    for pair in pairs {
        if pair.as_rule() == Rule::EOI {
            break;
        }
        match parser::read(pair) {
            Ok(ast) => match eval::eval(Expr::from(ast), &mut env) {
                Ok(expr) => println!("{expr}"),
                Err(e) => println!("{e:?}"),
            },
            Err(e) => println!("{e:?}"),
        }
    }
}
