use errors::DriverError;
use model::{Token, ast::*};
use std::iter::Peekable;

pub fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Program, DriverError> {
    let func = parse_function(tokens)?;

    if tokens.peek().is_some() {
        return Err(DriverError::with_err_msg("Unexpected trailing tokens"));
    }

    Ok(Program { func })
}

fn parse_function(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Func, DriverError> {
    use Token::*;

    if !matches!(tokens.next(), Some(IntKeyword)) {
        return Err(DriverError::with_err_msg("expected int return type"));
    }
    let Some(Identifier(ref fn_name)) = tokens.next() else {
        return Err(DriverError::with_err_msg("expected function name"));
    };
    if !matches!(tokens.next(), Some(OpenParenthesis)) {
        return Err(DriverError::with_err_msg("expected ("));
    }
    if !matches!(tokens.next(), Some(VoidKeyword)) {
        return Err(DriverError::with_err_msg("expected void keyword"));
    }
    if !matches!(tokens.next(), Some(CloseParenthesis)) {
        return Err(DriverError::with_err_msg("expected )"));
    }
    if !matches!(tokens.next(), Some(OpenBrace)) {
        return Err(DriverError::with_err_msg("expected {"));
    }

    let statement = parse_statement(tokens)?;

    if !matches!(tokens.next(), Some(CloseBrace)) {
        return Err(DriverError::with_err_msg("expected }"));
    }

    Ok(Func {
        name: fn_name.clone(),
        body: vec![statement],
    })
}

fn parse_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, DriverError> {
    use Token::*;

    if !matches!(tokens.next(), Some(ReturnKeyword)) {
        return Err(DriverError::with_err_msg("expected return keyword"));
    }

    let expr = parse_expr(tokens)?;

    if !matches!(tokens.next(), Some(Semicolon)) {
        return Err(DriverError::with_err_msg("expected ;"));
    }

    Ok(Statement::Return(expr))
}

fn parse_expr(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, DriverError> {
    use Token::*;

    let Some(next_token) = tokens.next() else {
        return Err(DriverError::with_err_msg("malformed expression"));
    };

    match next_token {
        Constant(val) => {
            // For now, we ignore non-base 10 integer literals, literals with an annotation of the
            // type (like 2l), etc.
            let val: i64 = val.parse().map_err(|e| {
                DriverError::with_err_msg(&format!("Failed to parse integral value {val}: {e}"))
            })?;

            Ok(Expr::Constant(val))
        }
        BitwiseNot | Minus => {
            let operator = if next_token == BitwiseNot {
                UnaryOperator::BitwiseNot
            } else {
                UnaryOperator::Minus
            };
            let inner_expr = parse_expr(tokens)?;
            Ok(Expr::UnaryOp {
                op: operator,
                expr: Box::new(inner_expr),
            })
        }
        OpenParenthesis => {
            let inner_expr = parse_expr(tokens)?;
            if !matches!(tokens.peek(), Some(CloseParenthesis)) {
                DriverError::with_err_msg("expected )");
            }
            let _ = tokens.next(); // Consume the close parenthesis
            Ok(inner_expr)
        }
        _ => Err(DriverError::with_err_msg("malformed expression")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(s: &str) -> Result<Peekable<impl Iterator<Item = Token>>, DriverError> {
        Ok(lexer::tokenize_str(s)?.into_iter().peekable())
    }

    #[test]
    fn test_parse_function() -> Result<(), DriverError> {
        assert_eq!(
            parse_function(&mut tokenize("int main(void) { return 2; }")?).unwrap(),
            Func {
                name: "main".to_string(),
                body: vec![Statement::Return(Expr::Constant(2))]
            }
        );

        assert_eq!(
            parse_function(&mut tokenize("void foo() {}")?).unwrap_err(),
            DriverError::with_err_msg("expected int return type")
        );

        assert!(parse_function(&mut tokenize("")?).is_err());

        assert_eq!(
            parse_function(&mut tokenize("int foo")?).unwrap_err(),
            DriverError::with_err_msg("expected (")
        );

        Ok(())
    }

    #[test]
    fn test_parse_simple_exprs() -> Result<(), DriverError> {
        parse_expr(&mut tokenize("2")?).unwrap();
        parse_expr(&mut tokenize("-2")?).unwrap();
        parse_expr(&mut tokenize("~2")?).unwrap();
        parse_expr(&mut tokenize("(2)")?).unwrap();
        parse_expr(&mut tokenize("(-2)")?).unwrap();
        parse_expr(&mut tokenize("~(-2)")?).unwrap();
        Ok(())
    }
}
