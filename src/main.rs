pub mod buildins;
mod eval;
mod lexer;
mod parser;
mod token;
mod types;

use std::{fs, path::Path, process::exit};

use clap::{Arg, Command};
use eval::{
    evaluator::Evaluator,
    objects::{Object, ResultObj},
};
use lexer::Lexer;
use parser::Parser;

pub const PANA_MIGUEL_ASCII: &str = include_str!("../assets/pana_miguel.txt");

fn main() {
    let mut cmd = Command::new("pana.exe")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Sebastian Gonzalez. <devsebasgr@gmail.com>")
        .about("Lenguaje de programacion en espanol!")
        .arg(Arg::new("archivo .pana").help("Direccion relativa del archivo .pana a ejecutar"));

    if let Some(file_path) = cmd.clone().get_matches().get_one::<String>("archivo .pana") {
        if file_path == "pana" {
            return println!("{}", PANA_MIGUEL_ASCII);
        }

        let file_path = Path::new(file_path);

        match file_path.extension() {
            Some(ext) => {
                if ext != "pana" {
                    eprintln!("Solo se puede ejecutar archivos .pana");
                    exit(1);
                }
            }
            None => {
                eprintln!("Solo se puede ejecutar archivos .pana");
                exit(1);
            }
        }

        let mut evaluator = Evaluator::new();
        let file_str = fs::read_to_string(file_path)
            .unwrap_or_else(|_| panic!("No es encotro el archivo {}", file_path.display()));

        let lexer = Lexer::new(file_str.chars().collect());
        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        // Imprir error a nivel de parser
        if let Some(err) = parser.error {
            eprintln!("{}", err);
            exit(1);
        }

        // Imprimir error de runtime
        if let ResultObj::Copy(Object::Error(msg)) = evaluator.eval_program(program) {
            eprintln!("{}", msg);
            exit(1);
        }
        return;
    }
    let help = cmd.render_long_help();
    println!("{help}");
}

#[cfg(test)]
mod test;
