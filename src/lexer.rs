use std::{iter::Peekable, str::CharIndices};

use crate::token::{keywords_to_tokens, Token};

pub struct Lexer<'a> {
    pub input: &'a str,
    pub iter: Peekable<CharIndices<'a>>,
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
        }
    }

    fn read_char(&mut self) -> Option<(usize, char)> {
        self.iter.next()
    }

    fn peek_char(&mut self) -> Option<&(usize, char)> {
        self.iter.peek()
    }

    fn read_identifier(&mut self, token: (usize, char)) -> &'a str {
        let start = token.0;
        let mut end = start + 1;
        while self
            .peek_char()
            .map_or_else(|| false, |(_, char)| is_alphabetic(*char))
        {
            self.read_char();
            end += 1;
        }
        // Regex "([^0-9]\w*)_(\w*)"gm
        // Para snake case
        &self.input[start..end]
    }

    fn read_number(&mut self, token: (usize, char)) -> &'a str {
        let start = token.0;
        let mut end = start + 1;

        while self
            .peek_char()
            .map_or_else(|| false, |(_, char)| char.is_numeric())
        {
            self.read_char();
            end += 1;
        }

        &self.input[start..end]
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

    fn read_string(&mut self, start: usize) -> String {
        let mut end = start + 1;

        while let Some((_, c)) = self.read_char() {
            if c == '"' || c == '\0' {
                break;
            }
            end += 1;
        }

        self.input[start + 1..end].to_owned()
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
            Some((index, '"')) => Token::String(self.read_string(index)),
            Some((index, char)) => {
                if is_alphabetic(char) {
                    let identifier = self.read_identifier((index, char));
                    return keywords_to_tokens(identifier);
                } else if char.is_numeric() {
                    return Token::Int(self.read_number((index, char)).parse().unwrap());
                }

                Token::Illegal(char)
            }
            None => Token::Eof,
        }
    }
}

fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
