use crate::eval::{Error, Expr, Result};

pub fn add(args: Vec<Expr>) -> Result<Expr> {
    let mut result = 0;
    for arg in args {
        if let Expr::Int(i) = arg {
            result += i;
        } else {
            return Err(Error::InvalidType {
                expected: String::new(),
                actual: String::new(),
            });
        }
    }

    Ok(Expr::Int(result))
}
