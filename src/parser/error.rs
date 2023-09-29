use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    Illegal(Token),
    IllegalMsg(String, usize, usize),
    MissingIn(usize, usize),
    MissingRange(usize, usize),
    MissingIdentifier(usize, usize),
    MissingAssign(usize, usize),
    MissingColon(usize, usize),
    MissingComma(usize, usize),
    MissingExpression(usize, usize),
    MissingSemiColon(usize, usize),
    MissingLeftBrace(usize, usize),
    MissingLeftParen(usize, usize),
    MissingRightParen(usize, usize),
    MissingRightBrace(usize, usize),
    MissingRightBracket(usize, usize),
}

pub fn set_parser_err_line_col(err: ParserError, line: usize, col: usize) -> ParserError {
    match err {
        ParserError::MissingIdentifier(_, _) => ParserError::MissingIdentifier(line, col),
        ParserError::MissingAssign(_, _) => ParserError::MissingAssign(line, col),
        ParserError::MissingColon(_, _) => ParserError::MissingColon(line, col),
        ParserError::MissingComma(_, _) => ParserError::MissingComma(line, col),
        ParserError::MissingExpression(_, _) => ParserError::MissingExpression(line, col),
        ParserError::MissingSemiColon(_, _) => ParserError::MissingSemiColon(line, col),
        ParserError::MissingLeftBrace(_, _) => ParserError::MissingLeftBrace(line, col),
        ParserError::MissingLeftParen(_, _) => ParserError::MissingLeftParen(line, col),
        ParserError::MissingRightParen(_, _) => ParserError::MissingRightParen(line, col),
        ParserError::MissingRightBrace(_, _) => ParserError::MissingRightBrace(line, col),
        ParserError::MissingRightBracket(_, _) => ParserError::MissingRightBracket(line, col),
        err => err,
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Illegal(token) => {
                write!(
                    f,
                    "{}",
                    create_syntax_err(
                        &format!("Se encontro un simbolo ilegal `{}`", token.r#type),
                        &token.line,
                        &token.col
                    )
                )
            }
            ParserError::MissingIdentifier(line, col) => {
                write!(
                    f,
                    "{}",
                    create_syntax_err("Falta el nombre de la variable", line, col)
                )
            }
            ParserError::MissingAssign(line, col) => {
                write!(
                    f,
                    "{}",
                    create_syntax_err("Falta el simbolo `=` de asignacion", line, col)
                )
            }
            ParserError::MissingExpression(line, col) => {
                write!(f, "{}", create_syntax_err("Falta la expresion", line, col))
            }
            ParserError::MissingSemiColon(line, col) => {
                write!(f, "{}", create_syntax_err("Falta el `;`", line, col))
            }
            ParserError::MissingLeftBrace(line, col) => {
                write!(f, "{}", create_syntax_err("Fata el `{`", line, col))
            }
            ParserError::MissingRightBrace(line, col) => {
                write!(f, "{}", create_syntax_err("Fata el `}`", line, col))
            }
            ParserError::MissingLeftParen(line, col) => {
                write!(f, "{}", create_syntax_err("Falta el `(`", line, col))
            }
            ParserError::MissingRightParen(line, col) => {
                write!(f, "{}", create_syntax_err("Falta el `)`", line, col))
            }
            ParserError::MissingRightBracket(line, col) => {
                write!(f, "{}", create_syntax_err("Falta el `]`", line, col))
            }
            ParserError::MissingColon(line, col) => {
                write!(f, "{}", create_syntax_err("Falta el `:`", line, col))
            }
            ParserError::MissingComma(line, col) => {
                write!(f, "{}", create_syntax_err("Falta el `'`", line, col))
            }
            ParserError::IllegalMsg(msg, line, col) => {
                write!(f, "{}", create_syntax_err(msg, line, col))
            }
            ParserError::MissingIn(line, col) => {
                write!(
                    f,
                    "{}",
                    create_syntax_err("Falta la palabra clave `en`", line, col)
                )
            }
            ParserError::MissingRange(line, col) => {
                write!(
                    f,
                    "{}",
                    create_syntax_err("Falta la palabra clave `rango`", line, col)
                )
            }
        }
    }
}

fn create_syntax_err(msg: &str, line: &usize, col: &usize) -> String {
    format!(
        "Error de sintaxis: {}. Linea {}, columna {}.",
        msg, line, col
    )
}
