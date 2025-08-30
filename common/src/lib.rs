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
