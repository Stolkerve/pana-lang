mod ast;
mod buildins;
mod environment;
mod evaluator;
mod lexer;
mod objects;
mod parser;
mod promp_theme;
mod repl;
mod token;

use clap::{Arg, Command};
use console::Term;
use evaluator::Evaluator;
use lexer::Lexer;
use objects::{Object, ResultObj};
use parser::Parser;
use repl::repl;
use std::{fs, io::Error};

pub const PANA_MIGUEL_ASCII: &str = include_str!("../assets/pana_miguel.txt");

fn main() -> Result<(), Error> {
    let matches = Command::new("Lenguaje de programacion Pana")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Sebastian Gonzalez. <devsebasgr@gmail.com>")
        .about("Lenguaje de programacion en espanol!")
        .arg(Arg::new("archivo"))
        .get_matches();

    let term = Term::stdout();

    if let Some(archivo) = matches.get_one::<String>("archivo") {
        if archivo == "pana" {
            term.write_line(PANA_MIGUEL_ASCII)?;
            return Ok(());
        }

        let mut evaluator = Evaluator::new();
        for line in fs::read_to_string(archivo)
            .unwrap_or_else(|_| panic!("No es encotro el archivo {}", archivo))
            .lines()
        {
            let lexer = Lexer::new(line);
            let mut parser = Parser::new(lexer);

            if !parser.errors.is_empty() {
                term.write_line(&format!(
                    "{}",
                    console::style(
                        parser
                            .errors
                            .iter()
                            .map(|e| e.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                    .red()
                ))?;
                return Ok(());
            }
            let program = parser.parse_program();
            if let ResultObj::Borrow(Object::Error(msg)) = evaluator.eval_program(program) {
                println!("{}", msg);
                return Ok(());
            }
        }
        return Ok(());
    }

    repl(term)
}
