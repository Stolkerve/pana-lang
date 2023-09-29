mod error;
pub mod expression;
pub mod statement;
use std::collections::HashMap;

use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

use self::{
    error::{set_parser_err_line_col, ParserError},
    expression::{ExprType, Expression, FnParams},
    statement::{BlockStatement, Statement},
};

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
enum Precedence {
    Lowest = 0,
    Equals = 1,      // ==
    LessGreater = 2, // < >
    SumSub = 3,      // + y -
    ProductDiv = 4,  // * y /
    Prefix = 5,      //-1
    Call = 6,        // foo()
    Index = 7,       // foo()
    Member = 8,      // foo()
}

fn to_tokens_precedence(token: &TokenType) -> Precedence {
    match token {
        TokenType::Plus => Precedence::SumSub,
        TokenType::Minus => Precedence::SumSub,
        TokenType::Slash => Precedence::ProductDiv,
        TokenType::Asterisk => Precedence::ProductDiv,
        TokenType::Eq => Precedence::Equals,
        TokenType::NotEq => Precedence::Equals,
        TokenType::Lt => Precedence::LessGreater,
        TokenType::Gt => Precedence::LessGreater,
        TokenType::LtEq => Precedence::LessGreater,
        TokenType::GtEq => Precedence::LessGreater,
        TokenType::LParen => Precedence::Call,
        TokenType::LBracket => Precedence::Index,
        TokenType::Dot => Precedence::Member,
        _ => Precedence::Lowest,
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    pub error: Option<ParserError>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            current_token: Token {
                r#type: TokenType::Eof,
                line: 0,
                col: 0,
            },
            peek_token: Token {
                r#type: TokenType::Eof,
                line: 0,
                col: 0,
            },
            error: None,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&self, token: TokenType) -> bool {
        self.current_token.r#type == token
    }

    fn peek_token_is(&self, token: TokenType) -> bool {
        self.peek_token.r#type == token
    }

    fn expected_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn peek_precedence(&mut self) -> Precedence {
        to_tokens_precedence(&self.peek_token.r#type)
    }

    fn current_precedence(&mut self) -> Precedence {
        to_tokens_precedence(&self.current_token.r#type)
    }

    fn check_semicolon(&mut self, expect_more: bool) -> Option<ParserError> {
        if self.peek_token_is(TokenType::SemiColon) {
            None
        } else {
            if self.peek_token_is(TokenType::Eof) || self.peek_token_is(TokenType::NewLine) {
                return Some(ParserError::MissingSemiColon(
                    self.current_token.line,
                    self.current_token.col,
                ));
            }
            if expect_more {
                return Some(ParserError::Illegal(self.peek_token.clone()));
            }
            None
        }
    }

    fn read_identifier(&self) -> Result<String, ParserError> {
        if let TokenType::Ident(ident) = self.peek_token.r#type.clone() {
            Ok(ident.to_string())
        } else {
            return Err(ParserError::MissingIdentifier(
                self.current_token.line,
                self.current_token.col,
            ));
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        self.next_token();
        self.next_token();

        while self.current_token.r#type != TokenType::Eof {
            if !self.current_token_is(TokenType::CommentLine)
                && !self.current_token_is(TokenType::NewLine)
            {
                match self.parse_statement() {
                    Ok(stmt) => statements.push(stmt),
                    Err(err) => {
                        self.error = Some(err);
                        break;
                    }
                }
            }
            self.next_token();
        }
        statements
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        // println!("{:?}, {:?}", self.current_token, self.peek_token);
        if self.current_token_is(TokenType::CommentLine) {
            self.next_token();
            return self.parse_statement();
        }
        match self.current_token.r#type {
            TokenType::Var => self.parse_var_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Func => self.parse_fn_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
        let mut statements = BlockStatement::default();

        self.next_token();

        while !self.current_token_is(TokenType::RBrace) {
            if self.current_token_is(TokenType::Eof) {
                return Err(ParserError::MissingRightBrace(
                    self.current_token.line,
                    self.current_token.col,
                ));
            }
            if !self.current_token_is(TokenType::NewLine) {
                match self.parse_statement() {
                    Ok(stmt) => {
                        statements.push(stmt);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            self.next_token();
        }

        Ok(statements)
    }

    fn parse_var_statement(&mut self) -> Result<Statement, ParserError> {
        let identifier = self.read_identifier()?;

        self.next_token();

        if !self.expected_peek(TokenType::Assign) {
            return Err(ParserError::MissingAssign(
                self.current_token.line,
                self.current_token.col + 1,
            ));
        }

        self.next_token();

        if self.current_token_is(TokenType::Eof) {
            return Err(ParserError::MissingExpression(
                self.current_token.line,
                self.current_token.col,
            ));
        }

        let expr = self.parse_expression(Precedence::Lowest)?;

        match self.check_semicolon(true) {
            Some(err) => {
                return Err(err);
            }
            None => {
                self.next_token();
            }
        }

        Ok(Statement::Var {
            name: identifier,
            value: expr,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;

        match self.check_semicolon(true) {
            Some(err) => {
                return Err(err);
            }
            None => {
                self.next_token();
            }
        }

        Ok(Statement::Return(expr))
    }

    fn parse_fn_statement(&mut self) -> Result<Statement, ParserError> {
        let identifier: String;

        if let TokenType::Ident(ident) = self.peek_token.r#type.clone() {
            identifier = ident.to_string();
        } else {
            return Err(ParserError::MissingIdentifier(
                self.peek_token.line,
                self.peek_token.col,
            ));
        }
        let line = self.current_token.line;
        let col = self.current_token.col;

        self.next_token();

        if !self.expected_peek(TokenType::LParen) {
            return Err(ParserError::MissingLeftParen(
                self.peek_token.line,
                self.peek_token.col,
            ));
        }

        let params = self.parse_fn_params()?;

        if !self.expected_peek(TokenType::LBrace) {
            return Err(ParserError::MissingLeftBrace(
                self.peek_token.line,
                self.peek_token.col,
            ));
        }

        let body = self.parse_block_statement()?;

        Ok(Statement::Fn {
            name: identifier,
            params,
            body,
            line,
            col,
        })
    }

    fn parse_fn_params(&mut self) -> Result<FnParams, ParserError> {
        let mut params = FnParams::default();

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Ok(params);
        }

        self.next_token();

        if let TokenType::Ident(ident) = self.current_token.clone().r#type {
            params.push(Expression::new(
                ExprType::Identifier(ident),
                self.current_token.line,
                self.current_token.col,
            ));
            while self.peek_token_is(TokenType::Comma) {
                self.next_token();
                self.next_token();

                if let TokenType::Ident(ident) = self.current_token.clone().r#type {
                    params.push(Expression::new(
                        ExprType::Identifier(ident),
                        self.current_token.line,
                        self.current_token.col,
                    ));
                } else {
                    return Err(ParserError::Illegal(self.current_token.clone()));
                }
            }
        } else {
            return Err(ParserError::Illegal(self.current_token.clone()));
        }

        if !self.expected_peek(TokenType::RParen) {
            return Err(ParserError::MissingRightParen(
                self.peek_token.line,
                self.peek_token.col,
            ));
        }

        Ok(params)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::SemiColon) {
            self.next_token();
        }

        Ok(Statement::Expression(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        // Operaciones prefix
        let mut left_expr = {
            match &self.current_token.r#type {
                // Literals values
                TokenType::Ident(ident) => self.parse_identifier(ident.clone()),
                TokenType::Numeric(numeric) => Ok(Expression::new(
                    ExprType::NumericLiteral(numeric.to_owned()),
                    self.current_token.line,
                    self.current_token.col,
                )),
                TokenType::True => Ok(Expression::new(
                    ExprType::BooleanLiteral(true),
                    self.current_token.line,
                    self.current_token.col,
                )),
                TokenType::False => Ok(Expression::new(
                    ExprType::BooleanLiteral(false),
                    self.current_token.line,
                    self.current_token.col,
                )),
                TokenType::String(string) => Ok(Expression::new(
                    ExprType::StringLiteral(string.to_string()),
                    self.current_token.line,
                    self.current_token.col,
                )),
                TokenType::Null => Ok(Expression::new(
                    ExprType::NullLiteral,
                    self.current_token.line,
                    self.current_token.col,
                )),

                // Prefix
                TokenType::Bang => self.parse_prefix_expression(),
                TokenType::Plus => self.parse_prefix_expression(),
                TokenType::Minus => self.parse_prefix_expression(),
                TokenType::LParen => self.parse_grouped_expression(),
                TokenType::LBracket => self.parse_array_literal(),
                TokenType::LBrace => self.parse_dictionary_literal(),

                TokenType::If => self.parse_if_expression(),
                TokenType::While => self.parse_while_loop(),
                TokenType::For => self.parse_range_loop(),
                TokenType::Func => self.parse_fn_literal(),
                TokenType::IllegalMsg(msg) => Err(ParserError::IllegalMsg(
                    msg.to_owned(),
                    self.current_token.line,
                    self.current_token.col,
                )),
                _ => Err(ParserError::Illegal(self.current_token.clone())),
            }
        };

        // Operaciones infix
        if left_expr.is_ok() {
            while !self.peek_token_is(TokenType::SemiColon)
                && (precedence as u32) < (self.peek_precedence() as u32)
            {
                // let peek = self.peek_token();
                match self.peek_token.r#type {
                    TokenType::Plus => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::Minus => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::Asterisk => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::Slash => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::Eq => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::NotEq => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::Lt => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::Gt => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::LtEq => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::GtEq => {
                        self.next_token();
                        left_expr = self.parse_infix_expression(left_expr.unwrap());
                    }
                    TokenType::LParen => {
                        self.next_token();
                        left_expr = self.parse_call_expression(left_expr.unwrap());
                    }
                    TokenType::LBracket => {
                        self.next_token();
                        left_expr = self.parse_index_expression(left_expr.unwrap());
                    }
                    _ => {
                        return Err(ParserError::Illegal(self.peek_token.clone()));
                    }
                }
            }
            return left_expr;
        }

        // Retornar prefix
        left_expr
    }

    fn parse_identifier(&mut self, ident: String) -> Result<Expression, ParserError> {
        if !self.peek_token_is(TokenType::Assign) {
            return Ok(Expression::new(
                ExprType::Identifier(ident),
                self.current_token.line,
                self.current_token.col,
            ));
        }

        self.next_token();
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;

        match self.check_semicolon(false) {
            Some(err) => {
                return Err(err);
            }
            None => {
                self.next_token();
            }
        }

        Ok(Expression::new(
            ExprType::Assignment {
                left: Box::new(Expression::new(
                    ExprType::Identifier(ident),
                    self.current_token.line,
                    self.current_token.col,
                )),
                right: Box::new(expr),
            },
            self.current_token.line,
            self.current_token.col,
        ))
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let op_token = self.current_token.clone();
        self.next_token();
        if let Ok(right_expr) = self.parse_expression(Precedence::Prefix) {
            let expr = Expression::new(
                ExprType::Prefix {
                    operator: op_token.r#type,
                    right: Box::new(right_expr),
                },
                self.current_token.line,
                self.current_token.col,
            );
            return Ok(expr);
        }
        Err(ParserError::MissingExpression(
            self.current_token.line,
            self.current_token.col,
        ))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParserError> {
        let precedence = self.current_precedence();

        let op_token = self.current_token.clone();
        self.next_token();

        let right = self.parse_expression(precedence)?;
        Ok(Expression::new(
            ExprType::Infix {
                left: Box::new(left),
                operator: op_token.r#type,
                right: Box::new(right),
            },
            self.current_token.line,
            self.current_token.col,
        ))
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let conditional_expr = self.parse_expression(Precedence::Lowest)?;

        if !self.expected_peek(TokenType::LBrace) {
            return Err(ParserError::MissingLeftBrace(
                self.current_token.line,
                self.current_token.col,
            ));
        }

        let consequence_stmts = self.parse_block_statement()?;

        let mut alternative_stmts = BlockStatement::default();
        if self.peek_token_is(TokenType::Else) {
            self.next_token();

            if !self.expected_peek(TokenType::LBrace) {
                return Err(ParserError::MissingLeftBrace(
                    self.peek_token.line,
                    self.peek_token.col,
                ));
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
        Ok(Expression::new(
            ExprType::If {
                condition: Box::new(conditional_expr),
                consequence: consequence_stmts,
                alternative: alternative_stmts,
            },
            self.current_token.line,
            self.current_token.col,
        ))
    }

    fn parse_fn_literal(&mut self) -> Result<Expression, ParserError> {
        if !self.expected_peek(TokenType::LParen) {
            return Err(ParserError::MissingLeftParen(
                self.peek_token.line,
                self.peek_token.col,
            ));
        }

        let params = self.parse_fn_params()?;

        if !self.expected_peek(TokenType::LBrace) {
            return Err(ParserError::MissingLeftBrace(
                self.peek_token.line,
                self.peek_token.col,
            ));
        }

        let body = self.parse_block_statement()?;

        Ok(Expression::new(
            ExprType::FnLiteral { body, params },
            self.current_token.line,
            self.current_token.col,
        ))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;
        if !self.expected_peek(TokenType::RParen) {
            return Err(ParserError::MissingRightParen(
                self.peek_token.line,
                self.peek_token.col,
            ));
        }
        Ok(expr)
    }

    fn parse_expression_list(
        &mut self,
        end: TokenType,
        err: ParserError,
    ) -> Result<Vec<Expression>, ParserError> {
        let mut args = FnParams::default();

        if self.peek_token_is(end.clone()) {
            self.next_token();
            return Ok(args);
        }

        self.next_token();
        args.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::Lowest)?);

            // if !self.peek_token_is(TokenType::Comma) && !self.peek_token_is(end.clone()) {
            //     println!("asd");
            //     return Err(ParserError::MissingComma(
            //         self.peek_token.line,
            //         self.peek_token.col,
            //     ));
            // }
        }

        if !self.expected_peek(end) {
            return Err(set_parser_err_line_col(
                err,
                self.peek_token.line,
                self.peek_token.col,
            ));
        }

        Ok(args)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParserError> {
        let arguments =
            self.parse_expression_list(TokenType::RParen, ParserError::MissingRightParen(0, 0));

        match arguments {
            Ok(arguments) => {
                if let Some(err) = self.check_semicolon(false) {
                    return Err(err);
                }

                Ok(Expression::new(
                    ExprType::Call {
                        function: Box::new(function),
                        arguments,
                    },
                    self.current_token.line,
                    self.current_token.col,
                ))
            }
            Err(err) => Err(err),
        }
    }

    fn parse_array_literal(&mut self) -> Result<Expression, ParserError> {
        let elements =
            self.parse_expression_list(TokenType::RBracket, ParserError::MissingRightBracket(0, 0));
        match elements {
            Ok(elements) => Ok(Expression::new(
                ExprType::ListLiteral { elements },
                self.current_token.line,
                self.current_token.col,
            )),
            Err(err) => Err(err),
        }
    }

    fn parse_dictionary_literal(&mut self) -> Result<Expression, ParserError> {
        let mut dictionary = HashMap::new();
        while !self.peek_token_is(TokenType::RBrace) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;
            if !self.expected_peek(TokenType::Colon) {
                return Err(ParserError::MissingColon(
                    self.peek_token.line,
                    self.peek_token.col,
                ));
            }
            if let ExprType::FnLiteral { .. } = key.r#type {
                return Err(ParserError::IllegalMsg(
                    "No se puede usar funciones anonimas como llaves".to_owned(),
                    self.current_token.line,
                    self.current_token.col,
                ));
            }
            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;

            dictionary.insert(key, value);

            if !self.peek_token_is(TokenType::RBrace) && !self.expected_peek(TokenType::Comma) {
                return Err(ParserError::MissingComma(
                    self.peek_token.line,
                    self.peek_token.col,
                ));
            }
        }

        if !self.expected_peek(TokenType::RBrace) {
            return Err(ParserError::MissingRightBrace(
                self.peek_token.line,
                self.peek_token.col,
            ));
        }
        Ok(Expression::new(
            ExprType::DictionaryLiteral { pairs: dictionary },
            self.current_token.line,
            self.current_token.col,
        ))
    }

    fn parse_index_expression(&mut self, left: Expression) -> Result<Expression, ParserError> {
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;

        if !self.expected_peek(TokenType::RBracket) {
            return Err(ParserError::MissingRightBracket(
                self.peek_token.line,
                self.peek_token.col,
            ));
        }

        if self.peek_token_is(TokenType::Assign) {
            self.next_token();
            self.next_token();

            let right = self.parse_expression(Precedence::Lowest)?;
            if !self.expected_peek(TokenType::SemiColon) {
                return Err(ParserError::MissingSemiColon(
                    self.peek_token.line,
                    self.peek_token.col,
                ));
            }
            return Ok(Expression::new(
                ExprType::Assignment {
                    left: Box::new(Expression::new(
                        ExprType::Index {
                            left: Box::new(left),
                            index: Box::new(index),
                        },
                        self.current_token.line,
                        self.current_token.col,
                    )),
                    right: Box::new(right),
                },
                self.current_token.line,
                self.current_token.col,
            ));
        }

        Ok(Expression::new(
            ExprType::Index {
                left: Box::new(left),
                index: Box::new(index),
            },
            self.current_token.line,
            self.current_token.col,
        ))
    }

    fn parse_while_loop(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let conditional_expr = self.parse_expression(Precedence::Lowest)?;

        if !self.expected_peek(TokenType::LBrace) {
            return Err(ParserError::MissingLeftBrace(
                self.current_token.line,
                self.current_token.col,
            ));
        }

        let consequence_stmts = self.parse_block_statement()?;

        Ok(Expression::new(
            ExprType::While {
                condition: Box::new(conditional_expr),
                body: consequence_stmts,
            },
            self.current_token.line,
            self.current_token.col,
        ))
    }

    fn parse_range_loop(&mut self) -> Result<Expression, ParserError> {
        let line = self.peek_token.line;
        let col = self.peek_token.col;
        let identifier = self.read_identifier()?;
        self.next_token();

        if !self.expected_peek(TokenType::In) {
            // TODO!! Cambiar a error adecuado
            return Err(ParserError::MissingIn(
                self.current_token.line,
                self.current_token.col,
            ));
        }

        if !self.expected_peek(TokenType::Range) {
            // TODO!! Cambiar a error adecuado
            return Err(ParserError::MissingRange(
                self.current_token.line,
                self.current_token.col,
            ));
        }
        if !self.expected_peek(TokenType::LParen) {
            // TODO!! Cambiar a error adecuado
            return Err(ParserError::MissingLeftParen(
                self.current_token.line,
                self.current_token.col,
            ));
        }

        let arguments =
            self.parse_expression_list(TokenType::RParen, ParserError::MissingRightParen(0, 0))?;

        if !self.expected_peek(TokenType::LBrace) {
            return Err(ParserError::MissingLeftBrace(
                self.current_token.line,
                self.current_token.col,
            ));
        }
        let consequence_stmts = self.parse_block_statement()?;

        Ok(Expression::new(
            ExprType::ForRange {
                ident: identifier,
                arguments,
                body: consequence_stmts,
            },
            line,
            col,
        ))
    }
}
