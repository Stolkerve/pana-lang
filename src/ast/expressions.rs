use std::fmt::Display;

use crate::{ast::statements::BlockStatement, token::Token};

pub type FnParams = Vec<Expression>;

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    IntLiteral(i64),
    BooleanLiteral(bool),
    StringLiteral(String),
    NullLiteral,
    FnLiteral {
        params: FnParams,
        body: BlockStatement,
    },
    ArrayLiteral {
        elements: Vec<Expression>
    },
    Index {
        left: Box<Expression>,
        index: Box<Expression>
    },
    Prefix {
        operator: Token,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: Token,
    },
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: BlockStatement,
    },
    Call {
        function: Box<Expression>, // fn literal o identifier
        arguments: FnParams,
    },
    Assignment {
        name: String,
        value: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::IntLiteral(int) => write!(f, "{}", int),
            Expression::Prefix { operator, right } => write!(f, "{}{}", operator, right),
            Expression::Infix {
                left,
                right,
                operator,
            } => write!(f, "({}{}{})", left, operator, right),
            Expression::BooleanLiteral(boolean) => write!(f, "{}", boolean),
            Expression::If { condition, .. } => write!(f, "if {} {{...}}", condition),
            Expression::FnLiteral { params, .. } => {
                write!(f, "fn({}) {{...}}", format_arguments(params))
            }
            Expression::Call {
                function,
                arguments,
            } => write!(f, "{}({})", function, format_arguments(arguments)),
            Expression::Assignment { name, value } => write!(f, "{} = {};", name, *value),
            Expression::StringLiteral(string) => write!(f, "\"{}\"", string),
            Expression::ArrayLiteral { elements } => write!(f, "[{}]", format_arguments(elements)),
            Expression::Index { left, index } => write!(f, "{}[{}]", *left, *index),
            Expression::NullLiteral => write!(f, "nulo"),
        }
    }
}

pub fn format_arguments(exprs: &[Expression]) -> String {
    exprs
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}
