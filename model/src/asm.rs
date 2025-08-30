use std::fmt;

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

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Operand::*;

        match self {
            Imm(v) => write!(f, "${v}")?,
            Register => write!(f, "%eax")?,
        }

        Ok(())
    }
}
