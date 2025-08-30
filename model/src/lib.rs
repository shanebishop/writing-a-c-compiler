#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(String),
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

/// Contains Abstract Syntax Tree model data structures
pub mod ast;

/// Contains data structures for representing assembly
pub mod asm;
