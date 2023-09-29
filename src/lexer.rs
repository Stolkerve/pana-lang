use crate::types::Numeric;
use regex::Regex;

use crate::token::{keywords_to_tokens, Token, TokenType};

#[derive(PartialEq)]
pub enum NumericType {
    Integers,
    Floats,
    Hexadecimal,
    Octadecimal,
    Binary,
}

pub struct Lexer {
    input: Vec<char>,
    current_pos: usize,
    read_pos: usize,
    current_char: char,
    line: usize,
    col: usize,
    identifier_regex: Regex,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            current_pos: 0,
            current_char: '\0',
            line: 1,
            read_pos: 0,
            col: 0,
            // Recuerda reconocer bar_ como erroreneo
            identifier_regex: Regex::new("^[a-zA-ZáéíóúñüÁÉÍÓÚÑÜ_][a-zA-Z0-9áéíóúñüÁÉÍÓÚÑÜ_]*$")
                .expect("No compilo el regex de los identificadores"),
        }
    }

    fn read_char(&mut self) {
        if let Some(c) = self.input.get(self.read_pos) {
            self.current_char = *c;
        } else {
            self.current_char = '\0';
        }

        self.current_pos = self.read_pos;
        self.read_pos += char::len_utf8(self.current_char);
        self.col += 1;
    }

    fn peek_char(&self) -> Option<&char> {
        self.input.get(self.read_pos)
    }

    fn skip_whitespace(&mut self) {
        while self.current_char == ' ' || self.current_char == '\t' || self.current_char == '\r'
        // || self.current_char == '\n'
        {
            // if self.current_char == '\n' {
            //     self.col = 0;
            //     self.line += 1;
            // }
            self.read_char();
        }
    }

    fn is_identifier(&self, c: char) -> bool {
        matches!(c,
            '0'..='9'
            | 'a'..='z'
            | 'A'..='Z'
            | '_'
            | 'á'
            | 'Á'
            | 'é'
            | 'É'
            | 'í'
            | 'Í'
            | 'ó'
            | 'Ó'
            | 'ú'
            | 'Ú'
            | 'ñ'
            | 'Ñ'
            | 'ü'
            | 'Ü'
        )
    }

    fn is_identifier_alpha(&self, c: char) -> bool {
        matches!(c,
            'a'..='z'
            | 'A'..='Z'
            | '_'
            | 'á'
            | 'Á'
            | 'é'
            | 'É'
            | 'í'
            | 'Í'
            | 'ó'
            | 'Ó'
            | 'ú'
            | 'Ú'
            | 'ñ'
            | 'Ñ'
            | 'ü'
            | 'Ü'
        )
    }

    fn check_numeric(&self, state: &NumericType, c: char) -> bool {
        match state {
            NumericType::Integers => c.is_ascii_digit(),
            NumericType::Floats => c.is_ascii_digit(),
            NumericType::Hexadecimal => c.is_ascii_hexdigit(),
            NumericType::Octadecimal => c as u16 >= 48 && c as u16 <= 55,
            NumericType::Binary => c == '0' || c == '1',
        }
    }

    // 0b0101100
    // 0o320
    // 0xFF
    fn read_number(&mut self) -> Token {
        let start = self.current_pos;
        let mut end = start + 1;
        let mut state = NumericType::Integers;

        let line = self.line;
        if let Some(c) = self.peek_char() {
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
            } else if self.is_identifier_alpha(c) {
                return Token::new(TokenType::Illegal(c), line, self.col);
            }
        }

        while let Some(c) = self.peek_char() {
            let c = *c;
            if !c.is_ascii_digit() && !c.is_ascii_hexdigit() {
                if c == '.' {
                    if state == NumericType::Floats {
                        return Token::new(TokenType::Illegal(c), line, self.col);
                    }
                    state = NumericType::Floats;
                    self.read_char();
                    end += 1;
                    continue;
                }
                break;
            }

            if !self.check_numeric(&state, c) {
                return Token::new(TokenType::Illegal(c), line, self.col); // Pensar en un mejor mensaje
            }
            self.read_char();
            end += 1;
        }

        let mut token = Token::new(TokenType::Numeric(Numeric::Int(0)), self.line, self.col);

        token.r#type = match state {
            NumericType::Integers => {
                TokenType::Numeric(Numeric::Int(self.input[start..end].iter().collect::<String>().parse().unwrap()))
            }
            NumericType::Floats => TokenType::Numeric(Numeric::Float(
                self.input[start..end].iter().collect::<String>().parse::<f64>().unwrap(),
            )),
            NumericType::Hexadecimal => {
                if let Ok(int) =
                    i64::from_str_radix(self.input[start..end].iter().collect::<String>().trim_start_matches("0x"), 16)
                {
                    TokenType::Numeric(Numeric::Int(int))
                } else {
                    TokenType::IllegalMsg("Formato de numero invalido".to_owned())
                }
            }
            NumericType::Octadecimal => {
                if let Ok(int) =
                    i64::from_str_radix(self.input[start..end].iter().collect::<String>().trim_start_matches("0o"), 8)
                {
                    TokenType::Numeric(Numeric::Int(int))
                } else {
                    TokenType::IllegalMsg("Formato de numero invalido".to_owned())
                }
            }
            NumericType::Binary => {
                if let Ok(int) =
                    i64::from_str_radix(self.input[start..end].iter().collect::<String>().trim_start_matches("0b"), 2)
                {
                    TokenType::Numeric(Numeric::Int(int))
                } else {
                    TokenType::Numeric(Numeric::Int(1))
                }
            }
        };

        token
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.current_pos;
        let mut end = start + 1;

        while self
            .peek_char()
            .map_or_else(|| false, |c| self.is_identifier(*c))
        {
            self.read_char();
            end += 1;
        }
        
        let str_size: usize = self.input[start..end].iter().fold(0, |c1, c2| c1 + char::len_utf8(*c2));

        let ident = &self.input[start..start + str_size].iter().collect::<String>();
        if self.identifier_regex.is_match(ident) {
            return Token::new(keywords_to_tokens(ident), self.line, self.col);
        }
        Token::new(
            TokenType::IllegalMsg(format!(
                "El formato del identificador {} es erroneo, debe ser snake case.",
                ident
            )),
            self.line,
            self.col,
        )
    }

    fn read_string(&mut self) -> Token {
        let start = self.current_pos;
        let mut end = start + 1;

        while self.current_char != '\0' {
            self.read_char();
            if self.current_char == '"' {
                return Token::new(
                    TokenType::String(self.input[start + 1..end].iter().collect::<String>()),
                    self.line,
                    self.col,
                );
            }
            end += 1;
        }
        Token::new(
            TokenType::IllegalMsg("Falta el simbolo `\"` para delimitar la cadena".to_string()),
            self.line,
            self.col,
        )
    }

    fn read_2chars_token(
        &mut self,
        second_char: char,
        posible_token: TokenType,
        default_token: TokenType,
    ) -> Token {
        if let Some(c) = self.peek_char() {
            if *c == second_char {
                self.read_char();
                return Token::new(posible_token, self.line, self.col);
            }
        }
        Token::new(default_token, self.line, self.col)
    }

    fn read_to_end_line(&mut self) -> Token {
        while let Some(c) = self.peek_char() {
            if *c == '\n' {
                self.col = 0;
                self.line += 1;
                self.read_char();
                break;
            }
            self.read_char();
        }
        Token::new(TokenType::CommentLine, self.line, 0)
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();
        self.skip_whitespace();
        match self.current_char {
            '=' => self.read_2chars_token('=', TokenType::Eq, TokenType::Assign),
            '+' => Token::new(TokenType::Plus, self.line, self.col),
            '-' => Token::new(TokenType::Minus, self.line, self.col),
            '/' => Token::new(TokenType::Slash, self.line, self.col),
            '*' => Token::new(TokenType::Asterisk, self.line, self.col),
            '!' => self.read_2chars_token('=', TokenType::NotEq, TokenType::Bang),
            '<' => self.read_2chars_token('=', TokenType::LtEq, TokenType::Lt),
            '>' => self.read_2chars_token('=', TokenType::GtEq, TokenType::Gt),
            ',' => Token::new(TokenType::Comma, self.line, self.col),
            ';' => Token::new(TokenType::SemiColon, self.line, self.col),
            '(' => Token::new(TokenType::LParen, self.line, self.col),
            ')' => Token::new(TokenType::RParen, self.line, self.col),
            '{' => Token::new(TokenType::LBrace, self.line, self.col),
            '}' => Token::new(TokenType::RBrace, self.line, self.col),
            '[' => Token::new(TokenType::LBracket, self.line, self.col),
            ']' => Token::new(TokenType::RBracket, self.line, self.col),
            '.' => Token::new(TokenType::Dot, self.line, self.col),
            '#' => self.read_to_end_line(),
            ':' => Token::new(TokenType::Colon, self.line, self.col),
            '"' => self.read_string(),
            '\n' => {
                self.col = 0;
                self.line += 1;
                Token::new(TokenType::NewLine, self.line, self.col)
            }
            '\0' => Token::new(TokenType::Eof, self.line, self.col),
            c => {
                if self.is_identifier_alpha(c) {
                    return self.read_identifier();
                } else if c.is_ascii_digit() {
                    return self.read_number();
                }
                Token::new(TokenType::Illegal(c), self.line, self.col)
            }
        }
    }
}
