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
    UnaryOp { op: UnaryOperator, expr: Box<Expr> },
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    BitwiseNot,
    Minus,
}
