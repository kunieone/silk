use crate::token::{Literal, Operator, Symbol, Token, TokenType};

fn parse_expression(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParserError> {
    let mut left = parse_term(tokens, pos)?;
    loop {
        match tokens[*pos] {
            Token {
                tp: TokenType::Operator(op),
                ..
            } if op == Operator::Plus || op == Operator::Minus => {
                *pos += 1;
                let right = parse_term(tokens, pos)?;
                left = Expr::BinaryOp(op, Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParserError> {
    let mut left = parse_factor(tokens, pos)?;
    loop {
        match tokens[*pos] {
            Token {
                tp: TokenType::Operator(op),
                ..
            } if op == Operator::Multiple || op == Operator::Divide => {
                *pos += 1;
                let right = parse_factor(tokens, pos)?;
                left = Expr::BinaryOp(op, Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_factor(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParserError> {
    match &tokens[*pos] {
        Token {
            tp: TokenType::Literal(lit),
            ..
        } => {
            *pos += 1;
            Ok(Expr::Literal(lit.clone()))
        }
        Token {
            tp: TokenType::Id(name),
            ..
        } => {
            *pos += 1;
            Ok(Expr::Variable(name.clone()))
        }
        Token {
            tp: TokenType::Symbol(Symbol::Lparen),
            ..
        } => {
            *pos += 1;
            let expr = parse_expression(tokens, pos)?;
            if tokens[*pos].tp != TokenType::Symbol(Symbol::Rparen) {
                return Err(ParserError::MismatchedParentheses);
            }
            *pos += 1;
            Ok(expr)
        }
        _ => return Err(ParserError::UnexpectedToken(tokens[*pos].clone())),
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Token),
    MismatchedParentheses,
    // other possible errors here
}
#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    BinaryOp(Operator, Box<Expr>, Box<Expr>),
    // other possible expressions here
}

#[test]
fn A_test() {
    trait Swimable {
        fn swim(self);
    }

    struct Duck();
    impl Swimable for Duck {
        fn swim(self) {
            println!("{:?}", "Duck is swimming!");
        }
    }

    struct Fish();
    impl Swimable for Fish {
        fn swim(self) {
            println!("{:?}", "Fish is swimming!");
        }
    }

    fn swim_then_sing<T: Swimable>(item: T) -> T {
        item
    }
    fn swim_then_sing2<T: Swimable>(item: T) -> impl Swimable {
        item
    }
}
