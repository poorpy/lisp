mod env;
mod eval;
mod parser;

use env::Env;
use eval::Expr;
use pest::Parser;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut env = Env::default();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if let Err(err) = rl.add_history_entry(line.as_str()) {
                    println!("failed to add history entry: {err}")
                }
                if let Err(err) = handle(line, &mut env) {
                    println!("failed to handle line: {err:?}")
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("readline error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn handle(input: String, env: &mut Env) -> anyhow::Result<()> {
    let pairs = parser::LispParser::parse(parser::Rule::lisp, &input)?;

    for pair in pairs {
        match parser::read(pair) {
            Ok(ast) => match eval::eval(Expr::from(ast), env) {
                Ok(expr) => println!("{expr}"),
                Err(e) => println!("{e:?}"),
            },
            Err(e) => println!("{e:?}"),
        }
    }

    Ok(())
}
