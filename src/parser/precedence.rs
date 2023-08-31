use crate::token::Token;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Precedence {
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

pub fn to_tokens_precedence(token: &Token) -> Precedence {
    match token {
        Token::Plus => Precedence::SumSub,
        Token::Sub => Precedence::SumSub,
        Token::Div => Precedence::ProductDiv,
        Token::Mul => Precedence::ProductDiv,
        Token::Eq => Precedence::Equals,
        Token::NotEq => Precedence::Equals,
        Token::Lt => Precedence::LessGreater,
        Token::Gt => Precedence::LessGreater,
        Token::LtEq => Precedence::LessGreater,
        Token::GtEq => Precedence::LessGreater,
        Token::LParen => Precedence::Call,
        Token::LBracket => Precedence::Index,
        Token::Dot => Precedence::Member,
        _ => Precedence::Lowest,
    }
}
