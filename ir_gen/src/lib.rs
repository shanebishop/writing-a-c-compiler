//! `ir_gen` is a module for transforming the AST
//! to the TACKY intermediate representation. TACKY is a
//! intermediate representation that the book author (Nora Sandler) came
//! up with. The name TACKY is derived from TAC, or three-
//! address code intermediate representation. The TACKY
//! name was made by the author for fun.

use model::ast;
use model::tacky;
use model::tacky::Instruction;
use model::tacky::UnaryOp;

/// Used to produce unique identifiers for temporary variables in TACKY representation
static mut TEMPORARY_COUNTER: usize = 0;

pub fn ast_to_tacky(ast: &ast::Program) -> Vec<Instruction> {
    func_to_tacky(&ast.func)
}

fn func_to_tacky(func: &ast::Func) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for statement in &func.body {
        instructions.extend_from_slice(&statement_to_tacky(statement));
    }
    instructions
}

fn statement_to_tacky(statement: &ast::Statement) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match statement {
        ast::Statement::Return(expr) => {
            let expr = expr_to_tacky(expr, &mut instructions);
            instructions.push(Instruction::Return(expr));
        }
    };
    instructions
}

fn expr_to_tacky(expr: &ast::Expr, instructions: &mut Vec<Instruction>) -> tacky::Expr {
    match expr {
        ast::Expr::Constant(c) => tacky::Expr::Constant(*c),
        ast::Expr::UnaryOp { op, expr } => {
            let src = expr_to_tacky(expr, instructions);
            let dst_name = make_temporary();
            let dst = tacky::Expr::Var(dst_name.clone());
            let tacky_op = convert_unary_op(op);
            instructions.push(Instruction::UnaryOp {
                op: tacky_op,
                val: src,
                var: dst_name,
            });
            dst
        }
    }
}

fn convert_unary_op(op: &ast::UnaryOperator) -> UnaryOp {
    match op {
        ast::UnaryOperator::BitwiseNot => UnaryOp::BitwiseNot,
        ast::UnaryOperator::Minus => UnaryOp::Minus,
    }
}

fn make_temporary() -> String {
    let temporary_var_name = format!("tmp.{}", unsafe { TEMPORARY_COUNTER });
    unsafe { TEMPORARY_COUNTER += 1 };
    temporary_var_name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_to_tacky() {
        let ast = ast::Statement::Return(ast::Expr::Constant(3));
        assert_eq!(
            statement_to_tacky(&ast),
            vec![Instruction::Return(tacky::Expr::Constant(3))]
        );

        let ast = ast::Statement::Return(ast::Expr::UnaryOp {
            op: ast::UnaryOperator::BitwiseNot,
            expr: Box::new(ast::Expr::Constant(2)),
        });
        assert_eq!(
            statement_to_tacky(&ast),
            vec![
                Instruction::UnaryOp {
                    op: tacky::UnaryOp::BitwiseNot,
                    val: tacky::Expr::Constant(2),
                    var: "tmp.0".to_string()
                },
                Instruction::Return(tacky::Expr::Var("tmp.0".to_string()))
            ]
        );

        let ast = ast::Statement::Return(ast::Expr::UnaryOp {
            op: ast::UnaryOperator::Minus,
            expr: Box::new(ast::Expr::UnaryOp {
                op: ast::UnaryOperator::BitwiseNot,
                expr: Box::new(ast::Expr::UnaryOp {
                    op: ast::UnaryOperator::Minus,
                    expr: Box::new(ast::Expr::Constant(8)),
                }),
            }),
        });
        assert_eq!(
            statement_to_tacky(&ast),
            vec![
                Instruction::UnaryOp {
                    op: tacky::UnaryOp::Minus,
                    val: tacky::Expr::Constant(8),
                    var: "tmp.1".to_string()
                },
                Instruction::UnaryOp {
                    op: tacky::UnaryOp::BitwiseNot,
                    val: tacky::Expr::Var("tmp.1".to_string()),
                    var: "tmp.2".to_string()
                },
                Instruction::UnaryOp {
                    op: tacky::UnaryOp::Minus,
                    val: tacky::Expr::Var("tmp.2".to_string()),
                    var: "tmp.3".to_string()
                },
                Instruction::Return(tacky::Expr::Var("tmp.3".to_string()))
            ]
        );
    }
}
