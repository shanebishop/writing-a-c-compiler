use model::{asm::*, ast};

pub fn codegen(ast: &ast::Program) -> Program {
    use Instruction::*;

    let func = &ast.func;
    let ast::Statement::Return(ast::Expr::Constant(ret_val)) = func.body[0];

    Program {
        func: Func {
            name: func.name.clone(),
            instructions: vec![
                Mov {
                    src: Operand::Imm(ret_val),
                    dst: Operand::Register,
                },
                Ret,
            ],
        },
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
