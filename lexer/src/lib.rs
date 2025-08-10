use regex::Regex;
use std::cmp;
use std::ffi::OsStr;
use std::fs;

use errors::DriverError;

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
struct TokenInfo {
    /// Resulting token
    token: Token,
    /// Length of token
    len: usize,
}

#[derive(Debug, PartialEq)]
pub struct LexError;

impl From<LexError> for DriverError {
    fn from(_: LexError) -> Self {
        todo!()
    }
}

pub fn tokenize(path: &OsStr) -> Result<Vec<Token>, DriverError> {
    let source = fs::read_to_string(path)?;
    Ok(tokenize_str(&source)?)
}

pub fn tokenize_str(input: &str) -> Result<Vec<Token>, LexError> {
    let mut input = input;
    let mut tokens = Vec::new();

    while !input.is_empty() {
        if input.starts_with(char::is_whitespace) {
            input = input.trim_start_matches(char::is_whitespace);
        } else {
            let Some(token_info) = find_token(input) else {
                return Err(LexError);
            };
            tokens.push(token_info.token);
            input = &input[cmp::min(token_info.len, input.len())..];
        }
    }

    Ok(tokens)
}

type LexerMapping = (Regex, fn(&str) -> Token);

/// Map for tokenizing. Maps from tokenizer regex to closure for generating the token from the
/// regex capture.
static LEXER_MAP: std::sync::LazyLock<[LexerMapping; 10]> = std::sync::LazyLock::new(lexer_map);

/// Produces the map to be used in `LEXER_MAP``.
///
/// Each regex must follow the pattern `\A(<to capture>)`. The `\A` is important so that
/// we only match the start of the string, rather than searching for a match in the
/// entire input string/file. The `(<to capture>)` part is important so that we
/// always have a capture, even when the closure to turn the capture into a `Token` does
/// not require the capture.
///
/// We need this function to work around unwraps not being allowed in static contexts.
fn lexer_map() -> [LexerMapping; 10] {
    [
        (Regex::new(r"\A(int\b)").unwrap(), |_| Token::IntKeyword),
        (Regex::new(r"\A(void\b)").unwrap(), |_| Token::VoidKeyword),
        (Regex::new(r"\A(return\b)").unwrap(), |_| {
            Token::ReturnKeyword
        }),
        (Regex::new(r"\A([a-zA-Z_]\w*\b)").unwrap(), |s| {
            Token::Identifier(s.to_owned())
        }),
        (Regex::new(r"\A([0-9]+\b)").unwrap(), |s| {
            Token::Constant(s.to_owned())
        }),
        (Regex::new(r"\A(\()").unwrap(), |_| Token::OpenParenthesis),
        (Regex::new(r"\A(\))").unwrap(), |_| Token::CloseParenthesis),
        (Regex::new(r"\A(\{)").unwrap(), |_| Token::OpenBrace),
        (Regex::new(r"\A(\})").unwrap(), |_| Token::CloseBrace),
        (Regex::new(r"\A(;)").unwrap(), |_| Token::Semicolon),
    ]
}

fn find_token(input: &str) -> Option<TokenInfo> {
    struct Match<'a> {
        match_: regex::Match<'a>,
        to_token: fn(&str) -> Token,
    }

    impl<'a> Match<'a> {
        pub fn len(&self) -> usize {
            self.match_.len()
        }
    }

    let mut longest_match = None;
    for (re, func) in &*LEXER_MAP {
        let Some(match_) = re.find(input) else {
            continue;
        };

        let match_ = Match {
            match_,
            to_token: *func,
        };

        if longest_match.is_none() {
            longest_match = Some(match_);
        } else if let Some(ref lm) = longest_match
            && match_.len() > lm.len()
        {
            longest_match = Some(match_);
        }
    }

    let longest_match = longest_match?;

    let token = (longest_match.to_token)(longest_match.match_.as_str());
    Some(TokenInfo {
        token,
        len: longest_match.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_token() {
        use Token::*;

        assert_eq!(find_token(""), None);
        assert_eq!(
            find_token("int "),
            Some(TokenInfo {
                token: IntKeyword,
                len: 3
            })
        );
        assert_eq!(
            find_token("int"),
            Some(TokenInfo {
                token: IntKeyword,
                len: 3
            })
        );
        assert_eq!(
            find_token("my_var"),
            Some(TokenInfo {
                token: Identifier("my_var".to_string()),
                len: 6
            })
        );
        assert_eq!(
            find_token("my_var2"),
            Some(TokenInfo {
                token: Identifier("my_var2".to_string()),
                len: 7
            })
        );
        assert_eq!(find_token("2my_var"), None); // Identifiers cannot start with numbers
        assert_eq!(
            find_token("void"),
            Some(TokenInfo {
                token: VoidKeyword,
                len: 4
            })
        );
        assert_eq!(
            find_token("return"),
            Some(TokenInfo {
                token: ReturnKeyword,
                len: 6
            })
        );
        assert_eq!(
            find_token("("),
            Some(TokenInfo {
                token: OpenParenthesis,
                len: 1
            })
        );
        assert_eq!(
            find_token(")"),
            Some(TokenInfo {
                token: CloseParenthesis,
                len: 1
            })
        );
        assert_eq!(
            find_token("{"),
            Some(TokenInfo {
                token: OpenBrace,
                len: 1
            })
        );
        assert_eq!(
            find_token("}"),
            Some(TokenInfo {
                token: CloseBrace,
                len: 1
            })
        );
        assert_eq!(
            find_token("123"),
            Some(TokenInfo {
                token: Constant("123".to_string()),
                len: 3
            })
        );
        assert_eq!(find_token("1_234"), None); // C, unlike some other languages, does not support underscores in integer literals
    }

    #[test]
    fn test_tokenize_str() {
        use Token::*;

        assert_eq!(tokenize_str("int"), Ok(vec![IntKeyword]));
        assert_eq!(
            tokenize_str("int foo     ;"),
            Ok(vec![IntKeyword, Identifier("foo".to_string()), Semicolon])
        );
        assert_eq!(
            tokenize_str("}()((99; foo int {;"),
            Ok(vec![
                CloseBrace,
                OpenParenthesis,
                CloseParenthesis,
                OpenParenthesis,
                OpenParenthesis,
                Constant("99".to_string()),
                Semicolon,
                Identifier("foo".to_string()),
                IntKeyword,
                OpenBrace,
                Semicolon
            ])
        );
    }

    #[test]
    fn test_tokenize_str_ugly_inputs() {
        use Token::*;

        assert_eq!(tokenize_str("55555555555555555504"), Ok(vec![Constant("55555555555555555504".to_string())]));
    }
}
