mod eval;
mod parser;

use pest::Parser;

use crate::parser::Ast;

fn main() {
    let input = std::fs::read_to_string("input.lisp").expect("cannot read file");
    let pairs =
        parser::LispParser::parse(parser::Rule::lisp, &input).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        let done = parser::read(pair);
        println!("{done:?}");
        if let Ok(Ast::Program(vec)) = done {
            for ast in vec {
                println!("{:?}", eval::eval(ast));
            }
        }
    }
}
