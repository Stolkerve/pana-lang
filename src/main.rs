mod ast;
mod buildins;
mod environment;
mod evaluator;
mod lexer;
mod objects;
mod parser;
mod repl;
mod token;

use repl::repl;

fn main() {
    repl().unwrap();
}
