#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Boolean(bool),
    Symbol(String),
    String(String),
    Number(f64),
}
