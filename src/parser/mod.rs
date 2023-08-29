pub mod errors;
pub mod precedence;

use std::iter::Peekable;

use crate::ast::{
    expressions::{Expression, FnParams},
    statements::{BlockStatement, Statement},
    Program,
};
use crate::{lexer::Lexer, token::Token};

use self::{
    errors::ParserError,
    precedence::{to_tokens_precedence, Precedence},
};

pub struct Parser<'a> {
    pub lexer: Peekable<Lexer<'a>>,
    pub current_token: Token,
    pub errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer: lexer.peekable(),
            current_token: Token::Illegal('\0'),
            errors: Vec::new(),
        };

        parser.next_token(); // init current token
        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(err) => {
                    self.errors.push(err);
                    break;
                }
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token {
            Token::Var => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Func => self.parse_fn_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let identifier: String;

        if let Token::Ident(ident) = self.peek_token() {
            identifier = ident.to_string();
        } else {
            return Err(ParserError::MissingIdentifier);
        }

        self.next_token();

        if !self.expected_peek(Token::Assign) {
            return Err(ParserError::MissingAssign);
        }

        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;

        let peek = self.peek_token();
        if peek == &Token::SemiColon || peek == &Token::Eof {
            self.next_token();
        } else {
            return Err(ParserError::Illegal(self.peek_token().clone()));
        }

        Ok(Statement::Var {
            name: identifier,
            value: expr,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(Token::SemiColon) {
            self.next_token();
        } else {
            return Err(ParserError::MissingSemiColon);
        }

        match expr {
            Ok(expr) => Ok(Statement::Return(expr)),
            Err(err) => Err(err),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(Token::SemiColon) {
            self.lexer.next();
        }

        match expr {
            Ok(expr) => Ok(Statement::Expression(expr)),
            Err(err) => Err(err),
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        // Operaciones prefix
        let mut left_expr = {
            match &self.current_token {
                // Literals values
                Token::Ident(ident) => Ok(Expression::Identifier(ident.clone())),
                Token::Int(int) => Ok(Expression::IntLiteral(*int)),
                Token::True => Ok(Expression::BooleanLiteral(true)),
                Token::False => Ok(Expression::BooleanLiteral(false)),

                // Prefix
                Token::Not => self.parse_prefix_expression(),
                Token::Plus => self.parse_prefix_expression(),
                Token::Sub => self.parse_prefix_expression(),
                Token::LParen => self.parse_grouped_expression(),

                // Contitions
                Token::If => self.parse_if_expression(),
                Token::Func => self.parse_fn_literal(),
                token => Err(ParserError::Illegal(token.clone())),
            }
        };

        // Operaciones infix
        if left_expr.is_ok() {
            while !self.peek_token_is(Token::SemiColon)
                && (precedence as u32) < (self.peek_precedence() as u32)
            {
                match self.peek_token() {
                    Token::Plus => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::Sub => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::Mul => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::Div => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::Eq => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::NotEq => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::Lt => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::Gt => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::LtEq => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::GtEq => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    Token::LParen => {
                        self.next_token();
                        left_expr = self.parse_call_expression(left_expr.unwrap());
                    }
                    token => {
                        return Err(ParserError::Illegal(token.clone()));
                    }
                }
            }
            return left_expr;
        }

        // Retornar prefix
        left_expr
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let op_token = self.current_token.clone();
        self.next_token();
        if let Ok(right_expr) = self.parse_expression(Precedence::Prefix) {
            let expr = Expression::Prefix {
                operator: op_token,
                right: Box::new(right_expr),
            };
            return Ok(expr);
        }
        Err(ParserError::MissingExpression)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParserError> {
        let precedence = self.current_precedence();

        let op_token = self.current_token.clone();
        self.next_token();

        let right = self.parse_expression(precedence);
        if let Ok(right) = right {
            return Ok(Expression::Infix {
                left: Box::new(left),
                operator: op_token,
                right: Box::new(right),
            });
        }
        right
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest);
        if !self.expected_peek(Token::RParen) {
            return Err(ParserError::MissingRightParen);
        }
        expr
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let conditional_expr = self.parse_expression(Precedence::Lowest)?;

        if !self.expected_peek(Token::LBrace) {
            return Err(ParserError::MissingLeftBrace);
        }

        let consequence_stmts = self.parse_block_statement()?;

        let mut alternative_stmts = BlockStatement::default();
        if self.peek_token_is(Token::Else) {
            self.next_token();

            if !self.expected_peek(Token::LBrace) {
                return Err(ParserError::MissingLeftBrace);
            }

            match self.parse_block_statement() {
                Ok(stmts) => {
                    alternative_stmts = stmts;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(Expression::If {
            condition: Box::new(conditional_expr),
            consequence: consequence_stmts,
            alternative: alternative_stmts,
        })
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
        let mut statements = BlockStatement::default();

        self.next_token();

        while !self.current_token_is(Token::RBrace) {
            if self.current_token_is(Token::Eof) {
                return Err(ParserError::MissingRightBrace);
            }
            match self.parse_statement() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(err) => {
                    return Err(err);
                }
            }
            self.next_token();
        }

        Ok(statements)
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token().clone();
        self.lexer.next();
    }

    fn peek_token(&mut self) -> &Token {
        self.lexer.peek().unwrap_or(&Token::Eof)
    }

    fn expected_peek(&mut self, token_type: Token) -> bool {
        if let Some(token) = self.lexer.peek() {
            if &token_type == token {
                self.next_token();
                return true;
            } else {
                return false;
            }
        }
        false
    }

    fn current_token_is(&mut self, token: Token) -> bool {
        self.current_token == token
    }

    fn peek_token_is(&mut self, token: Token) -> bool {
        if let Some(tok) = self.lexer.peek() {
            return tok == &token;
        }
        false
    }

    fn peek_precedence(&mut self) -> Precedence {
        to_tokens_precedence(self.peek_token())
    }

    fn current_precedence(&mut self) -> Precedence {
        to_tokens_precedence(&self.current_token)
    }

    fn parse_fn_literal(&mut self) -> Result<Expression, ParserError> {
        if !self.expected_peek(Token::LParen) {
            return Err(ParserError::MissingLeftParen);
        }

        let params = self.parse_fn_params()?;

        if !self.expected_peek(Token::LBrace) {
            return Err(ParserError::MissingLeftBrace);
        }

        let body = self.parse_block_statement()?;

        Ok(Expression::FnLiteral { body, params })
    }

    fn parse_fn_params(&mut self) -> Result<FnParams, ParserError> {
        let mut params = FnParams::default();

        if self.peek_token_is(Token::RParen) {
            self.next_token();
            return Ok(params);
        }

        self.next_token();

        if let Token::Ident(ident) = self.current_token.clone() {
            params.push(Expression::Identifier(ident));
            while self.peek_token_is(Token::Comma) {
                self.next_token();
                self.next_token();

                if let Token::Ident(ident) = self.current_token.clone() {
                    params.push(Expression::Identifier(ident));
                } else {
                    return Err(ParserError::Illegal(self.current_token.clone()));
                }
            }
        } else {
            return Err(ParserError::Illegal(self.current_token.clone()));
        }

        if !self.expected_peek(Token::RParen) {
            return Err(ParserError::MissingRightParen);
        }

        Ok(params)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParserError> {
        let arguments = self.parse_call_arguments()?;

        Ok(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_call_arguments(&mut self) -> Result<FnParams, ParserError> {
        let mut args = FnParams::default();

        if self.peek_token_is(Token::RParen) {
            self.next_token();
            return Ok(args);
        }

        self.next_token();
        args.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::Lowest)?);
        }

        if !self.expected_peek(Token::RParen) {
            return Err(ParserError::MissingRightParen);
        }

        Ok(args)
    }

    fn parse_fn_statement(&mut self) -> Result<Statement, ParserError> {
        let identifier: String;

        if let Token::Ident(ident) = self.peek_token() {
            identifier = ident.to_string();
        } else {
            return Err(ParserError::MissingIdentifier);
        }
        self.next_token();

        if !self.expected_peek(Token::LParen) {
            return Err(ParserError::MissingLeftParen);
        }

        let params = self.parse_fn_params()?;

        if !self.expected_peek(Token::LBrace) {
            return Err(ParserError::MissingLeftBrace);
        }

        let body = self.parse_block_statement()?;

        Ok(Statement::Fn {
            name: identifier,
            params,
            body,
        })
    }
}
