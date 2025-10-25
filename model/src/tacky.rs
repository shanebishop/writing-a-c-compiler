#[derive(Debug, PartialEq)]
pub struct Program;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(i64),
    Unary {
        op: UnaryOp,
        val: Box<Expr>,
        var: String,
    },
    Var(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    BitwiseNot,
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    UnaryOp { op: UnaryOp, val: Expr, var: String },
    Return(Expr),
}
