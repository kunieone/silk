use crate::token::TokenType;

fn is_terminal(t: TokenType) -> bool {
    match t {
        TokenType::Literal(_) => true,
        TokenType::Id(_) => true,
        TokenType::Keyword(_) => todo!(),
        TokenType::Symbol(_) => todo!(),
        TokenType::Operator(_) => todo!(),
    }
}
