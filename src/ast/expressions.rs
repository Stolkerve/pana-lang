use std::{collections::HashMap, fmt::Display};

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
    ListLiteral {
        elements: Vec<Expression>,
    },
    DictionaryLiteral {
        pairs: HashMap<Expression, Expression>,
    },
    Index {
        left: Box<Expression>,
        index: Box<Expression>,
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
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl Eq for Expression {}

impl std::hash::Hash for Expression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::IntLiteral(l0), Self::IntLiteral(r0)) => l0 == r0,
            (Self::BooleanLiteral(l0), Self::BooleanLiteral(r0)) => l0 == r0,
            (Self::StringLiteral(l0), Self::StringLiteral(r0)) => l0 == r0,
            (Self::FnLiteral { .. }, Self::FnLiteral { .. }) => {
                panic!("No se puede comparar funciones anonimas")
            }
            (
                Self::ListLiteral {
                    elements: l_elements,
                },
                Self::ListLiteral {
                    elements: r_elements,
                },
            ) => l_elements == r_elements,
            (
                Self::DictionaryLiteral { pairs: l_pairs },
                Self::DictionaryLiteral { pairs: r_pairs },
            ) => l_pairs == r_pairs,
            (Self::Index { .. }, Self::Index { .. }) => {
                panic!("No se puede comparar expresion de indexacion")
            }
            (
                Self::Prefix {
                    operator: l_operator,
                    right: l_right,
                },
                Self::Prefix {
                    operator: r_operator,
                    right: r_right,
                },
            ) => l_operator == r_operator && l_right == r_right,
            (
                Self::Infix {
                    left: l_left,
                    right: l_right,
                    operator: l_operator,
                },
                Self::Infix {
                    left: r_left,
                    right: r_right,
                    operator: r_operator,
                },
            ) => l_left == r_left && l_right == r_right && l_operator == r_operator,
            (Self::If { .. }, Self::If { .. }) => {
                panic!("No se puede comparar bloques condicionales")
            }
            (Self::Call { .. }, Self::Call { .. }) => panic!("No se puede comparar llamadas"),
            (Self::Assignment { .. }, Self::Assignment { .. }) => {
                panic!("No se puede comparar asignaciones")
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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
            Expression::Assignment { left, right } => write!(f, "{} = {};", *left, *right),
            Expression::StringLiteral(string) => write!(f, "\"{}\"", string),
            Expression::ListLiteral { elements } => write!(f, "[{}]", format_arguments(elements)),
            Expression::Index { left, index } => write!(f, "{}[{}]", *left, *index),
            Expression::NullLiteral => write!(f, "nulo"),
            Expression::DictionaryLiteral { pairs } => write!(
                f,
                "{{{}}}",
                pairs
                    .iter()
                    .map(|(x, y)| format!("{}: {}", x, y))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
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
