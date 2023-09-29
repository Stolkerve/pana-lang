mod eval;
mod lexer;
mod parser;
mod token;
mod types;

use std::fs;

use clap::{Arg, Command};
use eval::{
    evaluator::Evaluator,
    objects::{Object, ResultObj},
};
use lexer::Lexer;
use parser::Parser;

pub const PANA_MIGUEL_ASCII: &str = include_str!("../assets/pana_miguel.txt");

fn main() {
    let matches = Command::new("Lenguaje de programacion Pana")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Sebastian Gonzalez. <devsebasgr@gmail.com>")
        .about("Lenguaje de programacion en espanol!")
        .arg(Arg::new("archivo"))
        .get_matches();

    if let Some(file_path) = matches.get_one::<String>("archivo") {
        if file_path == "pana" {
            println!("{}", PANA_MIGUEL_ASCII);
            return;
        }

        let mut evaluator = Evaluator::new();
        let file_str = fs::read_to_string(file_path)
            .expect(&format!("No es encotro el archivo {}", file_path));

        let lexer = Lexer::new(file_str.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        // Imprir error a nivel de parser
        if let Some(err) = parser.error {
            println!("{}", err);
            return;
        }

        // Imprimir error de runtime
        if let ResultObj::Copy(Object::Error(msg)) = evaluator.eval_program(program) {
            println!("{}", msg);
            return;
        }
        return;
    }

    println!("Comandos xd");
}

#[cfg(test)]
mod test;
