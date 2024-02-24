use crate::eval::{Error, Expr, Result};

fn into_ints(vec: Vec<Expr>) -> Result<Vec<i64>> {
    vec.into_iter()
        .map(|e| {
            if let Expr::Int(i) = e {
                Ok(i)
            } else {
                Err(Error::InvalidType {
                    expected: Expr::Int(0).typename(),
                    actual: e.typename(),
                })
            }
        })
        .collect::<Result<Vec<i64>>>()
}

pub fn add(args: Vec<Expr>) -> Result<Expr> {
    Ok(Expr::Int(into_ints(args)?.iter().sum()))
}

pub fn sub(args: Vec<Expr>) -> Result<Expr> {
    let ints = into_ints(args)?;
    let result = ints
        .iter()
        .cloned()
        .reduce(|a, b| a - b)
        .ok_or(Error::BadArity {
            name: "sub".to_string(),
            expected: 2,
            actual: ints.len(),
        })?;

    Ok(Expr::Int(result))
}

pub fn mul(args: Vec<Expr>) -> Result<Expr> {
    let ints = into_ints(args)?;
    let result = ints
        .iter()
        .cloned()
        .reduce(|a, b| a * b)
        .ok_or(Error::BadArity {
            name: "mul".to_string(),
            expected: 2,
            actual: ints.len(),
        })?;

    Ok(Expr::Int(result))
}

pub fn div(args: Vec<Expr>) -> Result<Expr> {
    let ints = into_ints(args)?;
    let dividend = ints[0];
    let divisor = ints[1..]
        .iter()
        .cloned()
        .reduce(|a, b| a * b)
        .ok_or(Error::BadArity {
            name: "div".to_owned(),
            expected: 2,
            actual: ints.len(),
        })?;

    if divisor == 0 {
        return Err(Error::DivideByZero);
    }

    Ok(Expr::Int(dividend / divisor))
}
