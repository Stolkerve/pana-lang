use std::fmt::Display;

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum Token {
    Illegal(char),
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(i64),

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

    // Delimiters
    Comma,
    SemiColon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Func,
    Var,
    Return,
    If,
    Else,
    True,
    False,
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
            Token::Int(int) => write!(f, "{}", int),
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
