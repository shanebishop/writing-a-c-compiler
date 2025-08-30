#[derive(Debug, PartialEq)]
pub struct Program {
    pub func: Func,
}

#[derive(Debug, PartialEq)]
pub struct Func {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Imm(i64),
    Register,
}
