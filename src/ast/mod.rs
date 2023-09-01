pub mod expressions;
pub mod statements;

use crate::ast::statements::BlockStatement;

#[derive(Debug, Default)]
pub struct Program {
    pub statements: BlockStatement,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: BlockStatement::new(),
        }
    }
}
