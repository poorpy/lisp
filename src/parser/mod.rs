#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct LispParser;
