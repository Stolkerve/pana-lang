use crate::types::Numeric;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
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
    Minus,
    Asterisk,
    Slash,
    Bang,
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
    NewLine,

    // Keywords
    Func,
    Var,
    Return,
    If,
    Else,
    True,
    False,
    Null,
    While
}

// impl Eq for TokenType {
// }

// impl PartialEq for TokenType {
//     fn ne(&self, other: &Self) -> bool {
//         !self.eq(other)
//     }

//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (TokenType::Illegal(c), TokenType::Illegal(c2)) => c == c2,
//             (TokenType::IllegalMsg(msg), TokenType::IllegalMsg(msg2)) => msg == msg2,
//            (t, t2) =>  t == t2
//         }
//     }
// }

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Eq => write!(f, "=="),
            TokenType::NotEq => write!(f, "!="),
            TokenType::Lt => write!(f, "<"),
            TokenType::Gt => write!(f, ">"),
            TokenType::LtEq => write!(f, "<="),
            TokenType::GtEq => write!(f, ">="),
            TokenType::Func => write!(f, "fn"),
            TokenType::True => write!(f, "verdad"),
            TokenType::False => write!(f, "falso"),
            TokenType::Illegal(char) => write!(f, "{}", char),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::Ident(ident) => write!(f, "{}", ident),
            TokenType::Numeric(int) => write!(f, "{}", int),
            TokenType::Assign => write!(f, "="),
            TokenType::Comma => write!(f, ","),
            TokenType::SemiColon => write!(f, ";"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::Var => write!(f, "var"),
            TokenType::Return => write!(f, "retornar"),
            TokenType::If => write!(f, "si"),
            TokenType::Else => write!(f, "sino"),
            TokenType::String(string) => write!(f, "\"{}\"", string),
            TokenType::Dot => write!(f, "."),
            TokenType::LBracket => write!(f, "["),
            TokenType::RBracket => write!(f, "]"),
            TokenType::Null => write!(f, "nulo"),
            TokenType::CommentLine => write!(f, "#"),
            TokenType::Colon => write!(f, ":"),
            TokenType::IllegalMsg(msg) => write!(f, "{}", msg),
            TokenType::NewLine => write!(f, "\\n"),
            TokenType::While => write!(f, "mientras"),
        }
    }
}

#[allow(dead_code)]
pub fn keywords_to_tokens(v: &str) -> TokenType {
    match v {
        "var" => TokenType::Var,
        "fn" => TokenType::Func,
        "si" => TokenType::If,
        "sino" => TokenType::Else,
        "retornar" => TokenType::Return,
        "verdad" => TokenType::True,
        "falso" => TokenType::False,
        "nulo" => TokenType::Null,
        "para" => TokenType::IllegalMsg("La palabra clave `para` no esta implementada aun".to_owned()),
        "mientras" => TokenType::While,
        "en" => TokenType::IllegalMsg("La palabra clave `en` no esta implementada aun".to_owned()),
        "continuar" => TokenType::IllegalMsg("La palabra clave `continuar` no esta implementada aun".to_owned()),
        "romper" => TokenType::IllegalMsg("La palabra clave `romper` no esta implementada aun".to_owned()),
        _ => TokenType::Ident(v.to_owned()),
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub line: usize,
    pub col: usize,
}

#[allow(dead_code)]
impl Token {
    pub fn new(r#type: TokenType, line: usize, col: usize) -> Self {
        Self { r#type, line, col }
    }
}
