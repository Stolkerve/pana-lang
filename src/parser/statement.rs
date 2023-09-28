use super::expression::{Expression, FnParams};

pub type BlockStatement = Vec<Statement>;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Statement {
    Var {
        name: String,
        value: Expression,
    },
    Return(Expression),
    Expression(Expression),
    Fn {
        name: String,
        params: FnParams,
        body: BlockStatement,
        line: usize,
        col: usize,
    },
}
