use super::expressions::{Expression, FnParams};

pub type BlockStatement = Vec<Statement>;

#[derive(Debug, Clone)]
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
    },
}
