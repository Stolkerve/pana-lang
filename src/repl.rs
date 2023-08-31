use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser, PANA_MIGUEL_ASCII, objects::Object, promp_theme::Tema};
use std::{collections::VecDeque, io::Error};

pub fn repl(term: console::Term) -> Result<(), Error> {
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

    let theme = Tema {};
    loop {
        let line: String = dialoguer::Input::with_theme(&theme)
            .with_prompt(">> ")
            .allow_empty(true)
            .history_with(&mut history)
            .interact_text_on(&term)
            .unwrap();

        if line == "limpiar" {
            term.clear_screen()?;
            continue;
        } else if line == "salir" {
            std::process::exit(0);
        } else if line == "pana" {
            term.write_line(PANA_MIGUEL_ASCII)?;
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
        match evaluator.eval_program(program) {
            Object::Void => {} 
            obj => {
                term.write_line(&format!("{}", obj))?;
            }
        }
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
