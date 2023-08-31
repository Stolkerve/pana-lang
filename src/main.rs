mod ast;
mod buildins;
mod environment;
mod evaluator;
mod lexer;
mod objects;
mod parser;
mod repl;
mod token;

use std::{fs, io::Error};
use clap::{Command, Arg};
use console::Term;
use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use repl::repl;

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

        let buffer = fs::read_to_string(archivo).expect(&format!("No es encotro el archivo {}", archivo));
        let mut evaluator = Evaluator::new();
        let lexer = Lexer::new(&buffer);
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
        evaluator.eval_program(program);
        return Ok(());
    }

    clearscreen::clear().expect("Ok, esto no lo deberias ver. En tal caso, debes estar corriendo un OS que no es windows o basado en unix/linux");
    repl(term)
}
