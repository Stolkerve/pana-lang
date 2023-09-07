use std::fmt::Display;

use crate::types::Numeric;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Illegal(char),
    IllegalMsg(String),
    Eof,
    CommentLine,

    // Identifiers, literals
    Ident(String),
    Numeric(Numeric),
    String(String),

    // Operators
    Assign,
    Plus,
    Sub,
    Div,
    Mul,
    Not,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Dot,

    // Delimiters
    Comma,
    SemiColon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,

    // Keywords
    Func,
    Var,
    Return,
    If,
    Else,
    True,
    False,
    Null,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Div => write!(f, "/"),
            Token::Mul => write!(f, "*"),
            Token::Not => write!(f, "!"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::LtEq => write!(f, "<="),
            Token::GtEq => write!(f, ">="),
            Token::Func => write!(f, "fn"),
            Token::True => write!(f, "verdad"),
            Token::False => write!(f, "falso"),
            Token::Illegal(char) => write!(f, "{}", char),
            Token::Eof => write!(f, "EOF"),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Numeric(int) => write!(f, "{}", int),
            Token::Assign => write!(f, "="),
            Token::Comma => write!(f, ","),
            Token::SemiColon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Var => write!(f, "var"),
            Token::Return => write!(f, "retornar"),
            Token::If => write!(f, "si"),
            Token::Else => write!(f, "sino"),
            Token::String(string) => write!(f, "\"{}\"", string),
            Token::Dot => write!(f, "."),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Null => write!(f, "nulo"),
            Token::CommentLine => write!(f, "|"),
            Token::Colon => write!(f, ":"),
            Token::IllegalMsg(msg) => write!(f, "{}", msg),
        }
    }
}

pub fn keywords_to_tokens(v: &str) -> Token {
    match v {
        "var" => Token::Var,
        "fn" => Token::Func,
        "si" => Token::If,
        "sino" => Token::Else,
        "retornar" => Token::Return,
        "verdad" => Token::True,
        "falso" => Token::False,
        "nulo" => Token::Null,
        _ => Token::Ident(v.to_owned()),
    }
}

// #[derive(Debug)]
// pub struct Token<'a> {
//     pub token_type: TokenType<'a>,
//     pub pos: usize
// }

// impl PartialEq for Token {
//     fn eq(&self, other: &Self) -> bool {
//         (self.literal == other.literal) && (self.token_type == other.token_type)
//     }
// }

// impl Token {
//     pub fn new(literal: String, token_type: TokenType) -> Self {
//         Self {
//             literal,
//             token_type,
//         }
//     }
// }
