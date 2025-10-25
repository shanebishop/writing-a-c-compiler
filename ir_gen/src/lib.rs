//! `ir_gen` is a module for transforming the AST
//! to the TACKY intermediate representation. TACKY is a
//! intermediate representation that the book author (Nora Sandler) came
//! up with. The name TACKY is derived from TAC, or three-
//! address code intermediate representation. The TACKY
//! name was made by the author for fun.

use model::{ast, tacky};

pub fn ast_to_tacky(ast: &ast::Program) -> tacky::Program {
    println!("In ast_to_tacky");
    tacky::Program
}

#[cfg(test)]
mod tests {
    use super::*;
}
