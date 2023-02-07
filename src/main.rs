mod parser;

use pest::Parser;

fn main() {
    let input = std::fs::read_to_string("input.lisp").expect("cannot read file");
    let pairs =
        parser::LispParser::parse(parser::Rule::lisp, &input).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        println!("{:?}", parser::read(pair))
        // // A pair is a combination of the rule which matched and a span of input
        // println!("Rule:    {:?}", pair.as_rule());
        // println!("Span:    {:?}", pair.as_span());
        // println!("Text:    {}", pair.as_str());
        //
        // for inner in pair.into_inner() {
        //     println!("Rule:    {:?}", inner.as_rule());
        //     println!("Span:    {:?}", inner.as_span());
        //     println!("Text:    {}", inner.as_str());
        // }
    }
}
