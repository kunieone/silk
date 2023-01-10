use std::fmt::Display;

use crate::pos::Pos;

#[derive(Clone)]
pub struct Token {
    pub tp: TokenType,
    pub pos: Pos,
}

#[derive(Clone, Eq, PartialEq)]
pub enum TokenType {
    Literal(Literal),
    Id(String),
    Keyword(Keyword),
    Symbol(Symbol),
    Operator(Operator),
}

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

impl ToString for Symbol {
    fn to_string(&self) -> String {
        match self {
            Symbol::Crge => "\\n".to_string(),
            Symbol::SelfAt => "@".to_string(),
            Symbol::Lparen => "(".to_string(),
            Symbol::Rparen => ")".to_string(),
            Symbol::Lbracket => "[".to_string(),
            Symbol::Rbracket => "]".to_string(),
            Symbol::Lbrace => "{".to_string(),
            Symbol::Rbrace => "}".to_string(),
            Symbol::Comma => ",".to_string(),
            Symbol::Hash => "#".to_string(),
            Symbol::Qmark => "?".to_string(),
            Symbol::Colon => ":".to_string(),
            Symbol::Dot => ".".to_string(),
            Symbol::UnderLine => "_".to_string(),
            Symbol::Dollar => "$".to_string(),
            Symbol::LineComment => "//".to_string(),
            Symbol::MultiComment => "/* */".to_string(),
            Symbol::Arrow => "->".to_string(),
            Symbol::Impl => "::".to_string(),
            Symbol::Omit => "..".to_string(),
            Symbol::Inner => "__".to_string(),
        }
    }
}
impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Self::Ability => "ability".to_string(),
            Self::F => "f".to_string(),
            Self::For => "for".to_string(),
            Self::As => "as".to_string(),
            Self::Non => "NONE".to_string(),
            Self::If => "if".to_string(),
            Self::Else => "else".to_string(),
            Self::While => "while".to_string(),
            Self::Loop => "loop".to_string(),
            Self::SilkAnd => "and".to_string(),
            Self::SilkOr => "or".to_string(),
            Self::Enum => "enum".to_string(),
            Self::Box => "box".to_string(),
            Self::Int => "int".to_string(),
            Self::SilkString => "string".to_string(),
            Self::Char => "char".to_string(),
            Self::Obj => "obj".to_string(),
            Self::Ref => "ref".to_string(),
            Self::Add => "add".to_string(),
            Self::SilkMax => "max".to_string(),
            Self::SilkMin => "min".to_string(),
        }
    }
}
impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Plus => "+".to_string(),
            Operator::Minus => "-".to_string(),
            Operator::Multiple => "*".to_string(),
            Operator::Divide => "/".to_string(),
            Operator::Mod => "%".to_string(),
            Operator::Or => "|".to_string(),
            Operator::And => "&".to_string(),
            Operator::Xor => "^".to_string(),
            Operator::Assign => "=".to_string(),
            Operator::Big => ">".to_string(),
            Operator::Small => "<".to_string(),
            Operator::Increment => "++".to_string(),
            Operator::Decrement => "--".to_string(),
            Operator::Pow => "**".to_string(),
            Operator::Wave => "~".to_string(),
            Operator::Connect => "~~".to_string(),
            Operator::PlusAssign => "+=".to_string(),
            Operator::MinusAssign => "-=".to_string(),
            Operator::MultipleAssign => "*=".to_string(),
            Operator::DivideAssign => "/=".to_string(),
            Operator::ModAssign => "%=".to_string(),
            Operator::BigEq => ">=".to_string(),
            Operator::SmallEq => "<=".to_string(),
            Operator::TotalEq => "==".to_string(),
            Operator::MinAssign => "min=".to_string(),
            Operator::MaxAssign => "max=".to_string(),
            Operator::OrAssign => "|=".to_string(),
            Operator::AndAssign => "&=".to_string(),
            Operator::XorAssign => "^=".to_string(),
            Operator::ShlAssign => "<<=".to_string(),
            Operator::ShrAssign => ">>=".to_string(),
            Operator::Shl => "<<".to_string(),
            Operator::Shr => ">>".to_string(),
            Operator::Deduce => "=>".to_string(),
            Operator::SpaceShip => "<=>".to_string(),
        }
    }
}
