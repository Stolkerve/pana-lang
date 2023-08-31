use self::statements::BlockStatement;

pub mod expressions;
pub mod statements;

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
