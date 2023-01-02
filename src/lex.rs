use crate::pos::Foots;
use crate::token::Keyword::*;
use crate::token::Literal::*;
use crate::token::Operator::*;
use crate::token::Symbol::*;
use crate::token::TokenType::*;
use crate::token::*;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut pos = Foots::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = &chars.next() {
        // let mut pos = pos; // define a new local variable
        // pos.update(*c);
        pos.update(*c);
        match c {
            c if c.is_alphabetic() => {
                let mut ident: String = String::new();
                ident.push(*c);
                while let Some(c) = chars.peek() {
                    if c.is_whitespace() || (!c.is_alphanumeric() && *c != '_') {
                        break;
                    }
                    pos.update(*c); // update pos
                    ident.push(chars.next().unwrap());
                }
                if ident == "max" && chars.peek() == Some(&'=') {
                    ident.push('=');
                    pos.update('=');
                    chars.next();
                    tokens.push(Token::new(Operator(MaxAssign), pos.sign_pos()));
                    continue;
                } else if ident == "min" && chars.peek() == Some(&'=') {
                    ident.push('=');
                    pos.update('=');
                    chars.next();
                    tokens.push(Token::new(Operator(MinAssign), pos.sign_pos()));
                    continue;
                };
                match &ident[..] {
                    "ability" => tokens.push(Token::new(Keyword(Ability), pos.sign_pos())),
                    "f" => tokens.push(Token::new(Keyword(F), pos.sign_pos())),
                    "for" => tokens.push(Token::new(Keyword(For), pos.sign_pos())),
                    "as" => tokens.push(Token::new(Keyword(As), pos.sign_pos())),
                    "none" => tokens.push(Token::new(Keyword(Non), pos.sign_pos())),
                    "if" => tokens.push(Token::new(Keyword(If), pos.sign_pos())),
                    "else" => tokens.push(Token::new(Keyword(Else), pos.sign_pos())),
                    "while" => tokens.push(Token::new(Keyword(While), pos.sign_pos())),
                    "loop" => tokens.push(Token::new(Keyword(Loop), pos.sign_pos())),
                    "or" => tokens.push(Token::new(Keyword(SilkOr), pos.sign_pos())),
                    "and" => tokens.push(Token::new(Keyword(SilkAnd), pos.sign_pos())),
                    "enum" => tokens.push(Token::new(Keyword(Enum), pos.sign_pos())),
                    "box" => tokens.push(Token::new(Keyword(Box), pos.sign_pos())),
                    "int" => tokens.push(Token::new(Keyword(Int), pos.sign_pos())),
                    "str" => tokens.push(Token::new(Keyword(SilkString), pos.sign_pos())),
                    "char" => tokens.push(Token::new(Keyword(Char), pos.sign_pos())),
                    "obj" => tokens.push(Token::new(Keyword(Obj), pos.sign_pos())),
                    "ref" => tokens.push(Token::new(Keyword(Ref), pos.sign_pos())),
                    "add" => tokens.push(Token::new(Keyword(Add), pos.sign_pos())),
                    "max" => tokens.push(Token::new(Keyword(SilkMax), pos.sign_pos())),
                    "min" => tokens.push(Token::new(Keyword(SilkMin), pos.sign_pos())),
                    _ => tokens.push(Token::new(Id(ident), pos.sign_pos())),
                }
            }
            c if c.is_numeric() => {
                let mut number = String::new();
                number.push(*c);
                while let Some(c) = chars.peek() {
                    if !c.is_numeric() {
                        break;
                    }
                    number.push(chars.next().unwrap());
                }

                let num: i32 = match number.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        panic!("Could not parse as an i32");
                    }
                };
                tokens.push(Token::new(Literal(Integer(num)), pos.sign_pos()));
            }

            '"' => {
                let mut string_literal = String::new();
                let mut slash_mode = false;
                while let Some(c) = chars.next() {
                    pos.update(c); // update pos

                    if !slash_mode {
                        if c == '\n' {
                            panic!("Unescaped illegal character");
                        }
                        if c == '\\' {
                            slash_mode = true;
                        } else if c == '"' {
                            break;
                        } else {
                            string_literal.push(c);
                        }
                    } else {
                        slash_mode = false;
                        string_literal.push(c);
                    }
                }

                tokens.push(Token::new(Literal(Str(string_literal)), pos.sign_pos()));
            }
            '\n' => tokens.push(Token::new(Symbol(Crge), pos.sign_pos())),
            '@' => tokens.push(Token::new(Symbol(SelfAt), pos.sign_pos())),
            '(' => tokens.push(Token::new(Symbol(Lparen), pos.sign_pos())),
            ')' => tokens.push(Token::new(Symbol(Rparen), pos.sign_pos())),
            '[' => tokens.push(Token::new(Symbol(Lbracket), pos.sign_pos())),
            ']' => tokens.push(Token::new(Symbol(Rbracket), pos.sign_pos())),
            '{' => tokens.push(Token::new(Symbol(Lbrace), pos.sign_pos())),
            '}' => tokens.push(Token::new(Symbol(Rbrace), pos.sign_pos())),
            ',' => tokens.push(Token::new(Symbol(Comma), pos.sign_pos())),
            '#' => tokens.push(Token::new(Symbol(Hash), pos.sign_pos())),
            '?' => tokens.push(Token::new(Symbol(Qmark), pos.sign_pos())),
            // ':' => tokens.push(),
            ':' => {
                if let Some(':') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Symbol(Impl), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Symbol(Colon), pos.sign_pos()))
                }
            }
            '.' => {
                if let Some('.') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Symbol(Omit), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Symbol(Dot), pos.sign_pos()))
                }
            }

            '_' => {
                if let Some('_') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Symbol(Inner), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Symbol(UnderLine), pos.sign_pos()))
                }
            }
            '$' => tokens.push(Token::new(Symbol(Dollar), pos.sign_pos())),
            '+' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(PlusAssign), pos.sign_pos()))
                } else if let Some('+') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(Increment), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(Plus), pos.sign_pos()))
                }
            }
            '-' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(MinusAssign), pos.sign_pos()))
                } else if let Some('-') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(Decrement), pos.sign_pos()))
                } else if let Some('>') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Symbol(Arrow), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(Minus), pos.sign_pos()))
                }
            }
            '*' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(MultipleAssign), pos.sign_pos()))
                } else if let Some('*') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(Pow), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(Multiple), pos.sign_pos()))
                }
            }
            '/' => {
                if let Some('=') = chars.peek() {
                    // /=
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(DivideAssign), pos.sign_pos()))
                } else if let Some('/') = chars.peek() {
                    // 单行注释
                    chars.next();
                    pos.update(*c);
                    while let Some(c) = chars.next() {
                        pos.update(c);
                        if c == '\n' {
                            break;
                        }
                    }
                } else if let Some('*') = chars.peek() {
                    // 多行注释
                    chars.next();
                    pos.update(*c);
                    let mut level = 1;
                    while let Some(c) = chars.next() {
                        pos.update(c);
                        if c == '/' && chars.peek() == Some(&'*') {
                            chars.next();
                            pos.update(c);
                            level += 1;
                        } else if c == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            pos.update(c);
                            level -= 1;
                            if level == 0 {
                                break;
                            }
                        }
                    }
                } else {
                    // /
                    tokens.push(Token::new(Operator(Divide), pos.sign_pos()))
                }
            }

            '%' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(ModAssign), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(Mod), pos.sign_pos()))
                }
            }
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(TotalEq), pos.sign_pos()))
                } else if let Some('>') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(Deduce), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(Assign), pos.sign_pos()))
                }
            }
            '~' => {
                if let Some('~') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(Connect), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(Wave), pos.sign_pos()))
                }
            }

            '|' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(OrAssign), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(Or), pos.sign_pos()))
                }
            }
            '&' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(AndAssign), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(And), pos.sign_pos()))
                }
            }
            '^' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(XorAssign), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(Xor), pos.sign_pos()))
                }
            }
            '>' => {
                //> >= >> >>=
                if let Some('=') = chars.peek() {
                    //>=
                    chars.next();
                    pos.update(*c);
                    tokens.push(Token::new(Operator(BigEq), pos.sign_pos()))
                } else if let Some('>') = chars.peek() {
                    //>>
                    chars.next();
                    pos.update(*c);
                    if let Some('=') = chars.peek() {
                        //>>=
                        chars.next();
                        pos.update(*c);
                        tokens.push(Token::new(Operator(ShrAssign), pos.sign_pos()))
                    } else {
                        //>>
                        tokens.push(Token::new(Operator(Shr), pos.sign_pos()))
                    }
                } else {
                    // >
                    tokens.push(Token::new(Operator(Big), pos.sign_pos()))
                }
            } //
            '<' => {
                // < << <<= <= <=>
                if let Some('<') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    if let Some('=') = chars.peek() {
                        //<<=
                        chars.next();
                        pos.update(*c);

                        tokens.push(Token::new(Operator(ShlAssign), pos.sign_pos()))
                    } else {
                        //<<
                        tokens.push(Token::new(Operator(Shl), pos.sign_pos()))
                    }
                } else if let Some('=') = chars.peek() {
                    chars.next();
                    pos.update(*c);
                    if let Some('>') = chars.peek() {
                        chars.next();
                        pos.update(*c);
                        tokens.push(Token::new(Operator(SpaceShip), pos.sign_pos()))
                    }
                    tokens.push(Token::new(Operator(SmallEq), pos.sign_pos()))
                } else {
                    tokens.push(Token::new(Operator(Small), pos.sign_pos()))
                }
            }
            '\t' => {
                dbg!(&"出现了缩进!");
            }
            ' ' => {}
            _ => {
                // dbg!(c);
                // panic!("Giggity")
            }
        }
    }
    tokens
}
