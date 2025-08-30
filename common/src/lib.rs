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

#[derive(Debug, PartialEq)]
pub struct Program {
    pub func: Func,
}

#[derive(Debug, PartialEq)]
pub struct Func {
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Constant(i64),
}
