use std::{iter::Peekable, str::CharIndices};

use regex::Regex;

use crate::{
    token::{keywords_to_tokens, Token},
    types::Numeric,
};

pub struct Lexer<'a> {
    pub input: &'a str,
    pub iter: Peekable<CharIndices<'a>>,
    pub line: usize,
    identifier_regex: Regex,
}

#[derive(PartialEq)]
enum NumericType {
    Integers,
    Floats,
    Hexadecimal,
    Octadecimal,
    Binary,
}

fn check_numeric(state: &NumericType, c: char) -> bool {
    match state {
        NumericType::Integers => c.is_ascii_digit(),
        NumericType::Floats => c.is_ascii_digit(),
        NumericType::Hexadecimal => c.is_ascii_hexdigit(),
        NumericType::Octadecimal => c as u16 >= 48 && c as u16 <= 55,
        NumericType::Binary => c == '0' || c == '1',
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token != Token::Eof {
            return Some(token);
        }
        None
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            iter: input.char_indices().peekable(),
            line: 1,
            identifier_regex: Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*$")
                .expect("No compilo el regex de los identificadores"),
        }
    }

    fn read_char(&mut self) -> Option<(usize, char)> {
        self.iter.next()
    }

    fn peek_char(&mut self) -> Option<&(usize, char)> {
        self.iter.peek()
    }

    fn read_identifier(&mut self, token: (usize, char)) -> Token {
        let start = token.0;
        let mut end = start + 1;
        while self
            .peek_char()
            .map_or_else(|| false, |(_, char)| is_identifier(*char))
        {
            self.read_char();
            end += 1;
        }

        let ident = &self.input[start..end];
        if self.identifier_regex.is_match(ident) {
            return keywords_to_tokens(ident);
        }
        Token::IllegalMsg(format!(
            "El formato del identificador {} es erroneo, debe ser snake case. Ejemplo: hola_mundo.",
            ident
        ))
    }

    // 0b0101100
    // 0o320
    // 0xFF
    fn read_number(&mut self, token: (usize, char)) -> Token {
        let start = token.0;
        let mut end = start + 1;
        let mut state = NumericType::Integers;

        if let Some((_, c)) = self.peek_char() {
            let c = *c;
            if c == 'b' || c == 'o' || c == 'x' {
                state = if c == 'b' {
                    NumericType::Binary
                } else if c == 'o' {
                    NumericType::Octadecimal
                } else {
                    NumericType::Hexadecimal
                };
                self.read_char();
                end += 1;
            } else if c == '.' {
                state = NumericType::Floats;
                self.read_char();
                end += 1;
            } else if is_identifier(c) {
                return Token::IllegalMsg(
                    "Ningun identificador puede empezar con un numero".to_owned(),
                );
            }
        }

        while let Some((_, c)) = self.peek_char() {
            let c = *c;
            if !c.is_ascii_digit() && !c.is_ascii_hexdigit() {
                if c == '.' {
                    if state == NumericType::Floats {
                        return Token::Illegal(c);
                    }
                    state = NumericType::Floats;
                    self.read_char();
                    end += 1;
                    continue;
                }
                break;
            }

            if !check_numeric(&state, c) {
                return Token::Illegal(c); // Pensar en un mejor mensaje
            }
            self.read_char();
            end += 1;
        }

        match state {
            NumericType::Integers => {
                Token::Numeric(Numeric::Int(self.input[start..end].parse().unwrap()))
            }
            NumericType::Floats => Token::Numeric(Numeric::Float(
                self.input[start..end].parse::<f64>().unwrap(),
            )),
            NumericType::Hexadecimal => Token::Numeric(Numeric::Int(
                i64::from_str_radix(self.input[start..end].trim_start_matches("0x"), 16).unwrap(),
            )),
            NumericType::Octadecimal => Token::Numeric(Numeric::Int(
                i64::from_str_radix(self.input[start..end].trim_start_matches("0o"), 8).unwrap(),
            )),
            NumericType::Binary => Token::Numeric(Numeric::Int(
                i64::from_str_radix(self.input[start..end].trim_start_matches("0b"), 2).unwrap(),
            )),
        }

        // Token::Numeric(self.input[start..end].parse().unwrap())
    }

    fn read_2chars_token(
        &mut self,
        second_char: char,
        posible_token: Token,
        default_token: Token,
    ) -> Token {
        if let Some((_, char)) = self.peek_char() {
            if *char == second_char {
                self.read_char();
                return posible_token;
            }
        }
        default_token
    }

    fn eat_whitespace(&mut self) {
        while self
            .peek_char()
            .map_or_else(|| false, |(_, char)| char.is_whitespace())
        {
            self.read_char();
        }
    }

    fn read_string(&mut self, start: usize) -> Token {
        let mut end = start + 1;

        while let Some((_, c)) = self.read_char() {
            if c == '"' {
                break;
            }
            end += 1;
        }
        if self.peek_char().is_none() {
            return Token::IllegalMsg("Falta el simbolo `\"` para delimitar la cadena".to_string());
        }

        Token::String(self.input[start + 1..end].to_owned())
    }

    fn read_to_end(&mut self) -> Token {
        while self.read_char().is_some() {}
        Token::CommentLine
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let token = self.read_char();

        match token {
            Some((_, '=')) => self.read_2chars_token('=', Token::Eq, Token::Assign),
            Some((_, '+')) => Token::Plus,
            Some((_, '-')) => Token::Sub,
            Some((_, '/')) => Token::Div,
            Some((_, '*')) => Token::Mul,
            Some((_, '!')) => self.read_2chars_token('=', Token::NotEq, Token::Not),
            Some((_, '<')) => self.read_2chars_token('=', Token::LtEq, Token::Lt),
            Some((_, '>')) => self.read_2chars_token('=', Token::GtEq, Token::Gt),
            Some((_, ',')) => Token::Comma,
            Some((_, ';')) => Token::SemiColon,
            Some((_, '(')) => Token::LParen,
            Some((_, ')')) => Token::RParen,
            Some((_, '{')) => Token::LBrace,
            Some((_, '}')) => Token::RBrace,
            Some((_, '[')) => Token::LBracket,
            Some((_, ']')) => Token::RBracket,
            Some((_, '.')) => Token::Dot,
            Some((_, '|')) => self.read_to_end(),
            Some((_, ':')) => Token::Colon,
            Some((index, '"')) => self.read_string(index),
            Some((index, char)) => {
                if is_identifier(char) {
                    let identifier = self.read_identifier((index, char));
                    return identifier;
                } else if char.is_ascii_digit() {
                    return self.read_number((index, char));
                }

                Token::Illegal(char)
            }
            None => Token::Eof,
        }
    }
}

fn is_identifier(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
