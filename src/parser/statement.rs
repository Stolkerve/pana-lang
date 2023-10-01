use super::expression::{Expression, FnParams};

pub type BlockStatement = Vec<Statement>;

#[derive(Debug, Clone)]
pub enum Statement {
    Break(usize, usize),
    Continue(usize, usize),
    Var {
        name: String,
        value: Expression,
    },
    Return(Expression, usize, usize),
    Expression(Expression),
    Fn {
        name: String,
        params: FnParams,
        body: BlockStatement,
        line: usize,
        col: usize,
    },
}
