use std::{io::Error, collections::VecDeque};
use clap::{arg, Command, Arg};
use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser};

use console::Term;

const PANA_MIGUEL_ASCII: &'static str = include_str!("../assets/pana_miguel.txt");

pub fn repl() -> Result<(), Error> {
    let matches = Command::new("Lenguaje de programacion Pana")
        .version("0.1")
        .author("Sebastian Gonzalez. <devsebasgr@gmail.com>")
        .about("Lenguaje de programacion en espanol!")
        .arg(Arg::new("archivo"))
        .get_matches();

    let term = Term::stdout();
    clearscreen::clear().expect("");

    if let Some(archivo) = matches.get_one::<String>("archivo") {
        if archivo == "pana" {
            term.write_line(PANA_MIGUEL_ASCII)?;
        }
    }

    term.write_line(&format!(
        "{}",
        console::style("¡Bienvenido al lenguaje de programacion Pana!").bold()
    ))?;
    term.write_line(&format!(
        "{}",
        console::style("¡Escribe cuantos comandos quieras!\n").bold()
    ))?;

    let mut history = History::new();
    let mut evaluator = Evaluator::new();

    loop {
        let line: String = dialoguer::Input::new()
            .with_prompt(">>")
            .allow_empty(true)
            .history_with(&mut history)
            .interact_text_on(&term)
            .unwrap();

        if line == "limpiar" {
            term.clear_screen()?;
            continue;
        } else if line == "salir" {
            std::process::exit(0);
        }

        let lexer = Lexer::new(line.as_str());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

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
            continue;
        }
        term.write_line(&format!("{}", evaluator.eval_program(program)))?;
    }
}

struct History {
    max: usize,
    history: VecDeque<String>,
}

impl History {
    fn new() -> Self {
        Self {
            max: 10000,
            history: VecDeque::new(),
        }
    }
}

impl<T: ToString> dialoguer::History<T> for History {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        if self.history.len() == self.max {
            self.history.pop_back();
        }
        self.history.push_front(val.to_string());
    }
}