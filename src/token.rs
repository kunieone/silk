use std::fmt::Display;

use crate::pos::Pos;

#[derive(Clone)]
pub struct Token {
    pub tp: TokenType,
    pub pos: Pos,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?}>", self.tp)
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?},{:?}>", self.tp, self.pos.tup())
    }
}

impl Token {
    pub fn new(tp: TokenType, pos: Pos) -> Self {
        Self { tp, pos }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.tp == other.tp
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum TokenType {
    Literal(Literal),
    Id(String),
    Keyword(Keyword),
    Symbol(Symbol),
    Operator(Operator),
}

impl std::fmt::Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
            Self::Id(arg0) => f.debug_tuple("Id").field(arg0).finish(),
            Self::Keyword(arg0) => f.debug_tuple("Keyword").field(arg0).finish(),
            Self::Symbol(arg0) => f.debug_tuple("Symbol").field(arg0).finish(),
            Self::Operator(arg0) => f.debug_tuple("Operator").field(arg0).finish(),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => f.debug_tuple("literal").field(arg0).finish(),
            Self::Id(arg0) => f.debug_tuple("identifier").field(arg0).finish(),
            Self::Keyword(arg0) => f.debug_tuple("keyword").field(arg0).finish(),
            Self::Symbol(arg0) => f.debug_tuple("symbol").field(arg0).finish(),
            Self::Operator(arg0) => f.debug_tuple("operator").field(arg0).finish(),
        }
    }
}

// fn is_same_enum(a: TokenType, b: TokenType) -> bool {
//     std::mem::discriminant(&a) == std::mem::discriminant(&b)
// }

#[derive(Clone, Debug, PartialEq, Eq)]

pub enum Literal {
    Str(String), // str
    Integer(i32),
    Float(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum Keyword {
    Ability,    // ability
    F,          // f
    For,        // for
    As,         // as
    Non,        // NONE
    If,         // if
    Else,       // else
    While,      // while
    Loop,       // loop
    SilkAnd,    // and
    SilkOr,     // or
    Enum,       // enum
    Box,        // box
    Int,        // int
    SilkString, // string
    Char,       // char
    Obj,        // obj
    Ref,        // ref
    Add,        // add
    SilkMax,    // max
    SilkMin,    // min
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum Symbol {
    Crge,         // \n
    SelfAt,       // @
    Lparen,       // (
    Rparen,       // )
    Lbracket,     // [
    Rbracket,     // ]
    Lbrace,       // {
    Rbrace,       // }
    Comma,        // ,
    Hash,         // #
    Qmark,        // ?
    Colon,        // :
    Dot,          // .
    UnderLine,    // _
    Dollar,       // $
    LineComment,  // //
    MultiComment, // /* */

    Arrow, // ->
    Impl,  // ::
    Omit,  // ..
    Inner, // __
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum Operator {
    Plus,     // +
    Minus,    // -
    Multiple, // *
    Divide,   // /
    Mod,      // %
    Or,       // |
    And,      // &
    Xor,      // ^
    Assign,   // =
    Big,      // >
    Small,    // <

    Increment,      // ++
    Decrement,      // --
    Pow,            // **
    Wave,           // ~
    Connect,        // ~~
    PlusAssign,     // +=
    MinusAssign,    // -=
    MultipleAssign, // *=
    DivideAssign,   // /=
    ModAssign,      // %=
    BigEq,          // >=
    SmallEq,        // <=
    TotalEq,        // ==
    MinAssign,      // min=
    MaxAssign,      // max=
    OrAssign,       // |=
    AndAssign,      // &=
    XorAssign,      // ^=
    ShlAssign,      // <<=
    ShrAssign,      // >>=
    Shl,            // <<
    Shr,            // >>
    Deduce,         // =>
    SpaceShip,      // <=>
}
