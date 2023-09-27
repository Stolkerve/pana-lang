use std::{collections::HashMap, fmt::Display};
use std::hash::Hash;

use crate::{token::TokenType, types::Numeric};

use super::statement::BlockStatement;

#[derive(Clone, Debug, Eq)]
pub struct Expression {
    pub r#type: ExprType,
    pub line: usize,
    pub col: usize
}

impl Expression {
    pub fn new(r#type: ExprType, line: usize, col: usize) -> Self { Self { r#type, line, col } }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.line == other.line && self.col == other.col
    }
}

impl Hash for Expression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r#type.hash(state);
        self.line.hash(state);
        self.col.hash(state);
    }
}

#[allow(dead_code)]
pub fn format_arguments(exprs: &[Expression]) -> String {
    exprs
        .iter()
        .map(|x| x.r#type.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

pub type FnParams = Vec<Expression>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ExprType {
    Identifier(String),
    NumericLiteral(Numeric),
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
        operator: TokenType,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: TokenType,
    },
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: BlockStatement,
    },
    While {
        condition: Box<Expression>,
        body: BlockStatement,
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

impl Eq for ExprType {}

impl std::hash::Hash for ExprType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl PartialEq for ExprType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::NumericLiteral(l0), Self::NumericLiteral(r0)) => l0 == r0,
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
            ) => l_operator == r_operator && l_right.r#type == r_right.r#type,
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
            ) => l_left.r#type == r_left.r#type && l_right.r#type == r_right.r#type && l_operator == r_operator,
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

impl Display for ExprType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprType::Identifier(ident) => write!(f, "{}", ident),
            ExprType::NumericLiteral(int) => write!(f, "{}", int),
            ExprType::Prefix { operator, right } => write!(f, "{}{}", operator, right.r#type),
            ExprType::Infix {
                left,
                right,
                operator,
            } => write!(f, "({}{}{})", left.r#type, operator, right.r#type),
            ExprType::BooleanLiteral(boolean) => write!(f, "{}", boolean),
            ExprType::If { condition, .. } => write!(f, "si {} {{...}}", condition.r#type),
            ExprType::FnLiteral { params, .. } => {
                write!(f, "fn({}) {{...}}", format_arguments(params))
            }
            ExprType::Call {
                function,
                arguments,
            } => write!(f, "{}({})", function.r#type, format_arguments(arguments)),
            ExprType::Assignment { left, right } => write!(f, "{} = {};", left.r#type, right.r#type),
            ExprType::StringLiteral(string) => write!(f, "\"{}\"", string),
            ExprType::ListLiteral { elements } => write!(f, "[{}]", format_arguments(elements)),
            ExprType::Index { left, index } => write!(f, "{}[{}]", left.r#type, index.r#type),
            ExprType::NullLiteral => write!(f, "nulo"),
            ExprType::DictionaryLiteral { pairs } => write!(
                f,
                "{{{}}}",
                pairs
                    .iter()
                    .map(|(x, y)| format!("{}: {}", x.r#type, y.r#type))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            ExprType::While { condition, .. } => write!(f, "mientras {} {{...}}", condition.r#type),
        }
    }
}