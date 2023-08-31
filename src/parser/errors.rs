use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    Illegal(Token),
    MissingIdentifier,
    MissingAssign,
    MissingExpression,
    MissingSemiColon,
    MissingLeftBrace,
    MissingLeftParen,
    MissingRightParen,
    MissingRightBrace,
    MissingRightBracket,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Illegal(token) => write!(f, "Se encontro un simbolo ilegal: {}", token),
            ParserError::MissingIdentifier => write!(f, "Falta el nombre de la variable",),
            ParserError::MissingAssign => write!(f, "Falta el simbolo `=` de asignacion"),
            ParserError::MissingExpression => write!(f, "Falta la expresion"),
            ParserError::MissingSemiColon => write!(f, "Falta el `;` al final de la expresion"),
            ParserError::MissingLeftBrace => write!(f, "Fata el `{{`"),
            ParserError::MissingRightBrace => write!(f, "Fata el `}}`"),
            ParserError::MissingLeftParen => write!(f, "Falta el `(`"),
            ParserError::MissingRightParen => write!(f, "Falta el )"),
            ParserError::MissingRightBracket => write!(f, "Falta el ]"),
        }
    }
}
