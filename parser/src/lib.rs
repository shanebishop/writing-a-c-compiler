use common::*;
use errors::DriverError;

pub fn parse(tokens: &[Token]) -> Result<Program, DriverError> {
    Ok(Program {
        func: parse_function(tokens)?,
    })
}

fn parse_function(tokens: &[Token]) -> Result<Func, DriverError> {
    use Token::*;

    if tokens.len() < 6 {
        return Err(DriverError::with_err_msg(
            "insufficient number of tokens for function definition",
        ));
    }

    if !matches!(tokens[0], IntKeyword) {
        return Err(DriverError::with_err_msg("expected int return type"));
    }
    let Identifier(ref fn_name) = tokens[1] else {
        return Err(DriverError::with_err_msg("expected function name"));
    };
    if !matches!(tokens[2], OpenParenthesis) {
        return Err(DriverError::with_err_msg("expected open parenthesis"));
    }
    if !matches!(tokens[3], VoidKeyword) {
        return Err(DriverError::with_err_msg("expected void keyword"));
    }
    if !matches!(tokens[4], CloseParenthesis) {
        return Err(DriverError::with_err_msg("expected close parenthesis"));
    }
    if !matches!(tokens[5], OpenBrace) {
        return Err(DriverError::with_err_msg("expected open brace"));
    }
    #[allow(clippy::unwrap_used)] // Check for tokens being non-empty is above
    if !matches!(tokens.last().unwrap(), CloseBrace) {
        return Err(DriverError::with_err_msg("expected close brace"));
    }

    let statement = parse_statement(&tokens[6..tokens.len() - 1])?;

    Ok(Func {
        name: fn_name.clone(),
        body: vec![statement],
    })
}

fn parse_statement(tokens: &[Token]) -> Result<Statement, DriverError> {
    use Token::*;

    if tokens.len() < 3 {
        return Err(DriverError::with_err_msg(
            "insufficient number of tokens for statement",
        ));
    }

    if !matches!(tokens[0], ReturnKeyword) {
        return Err(DriverError::with_err_msg("expected return keyword"));
    }
    #[allow(clippy::unwrap_used)] // tokens length check above
    if !matches!(tokens.last().unwrap(), Semicolon) {
        return Err(DriverError::with_err_msg("expected semi-colon"));
    }

    let expr = parse_expr(&tokens[1..2])?;
    Ok(Statement::Return(expr))
}

fn parse_expr(tokens: &[Token]) -> Result<Expr, DriverError> {
    use Token::*;

    let &[Constant(ref val)] = tokens else {
        return Err(DriverError::with_err_msg("illegal expression"));
    };

    // For now, we ignore non-base 10 integer literals, literals with an annotation of the
    // type (like 2l), etc.
    let val: i64 = val.parse().map_err(|e| {
        DriverError::with_err_msg(&format!("Failed to parse integral value {val}: {e}"))
    })?;

    Ok(Expr::Constant(val))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::tokenize_str;

    #[test]
    fn test_parse_function() -> Result<(), DriverError> {
        assert_eq!(
            parse_function(&tokenize_str("int main(void) { return 2; }")?)?,
            Func {
                name: "main".to_string(),
                body: vec![Statement::Return(Expr::Constant(2))]
            }
        );

        assert_eq!(
            parse_function(&tokenize_str("void foo() {}")?).unwrap_err(),
            DriverError::with_err_msg("expected int return type")
        );

        assert_eq!(
            parse_function(&[]).unwrap_err(),
            DriverError::with_err_msg("insufficient number of tokens for function definition")
        );

        assert_eq!(
            parse_function(&tokenize_str("int foo")?).unwrap_err(),
            DriverError::with_err_msg("insufficient number of tokens for function definition")
        );

        Ok(())
    }
}
