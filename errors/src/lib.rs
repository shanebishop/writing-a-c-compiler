use std::borrow::Cow;

/// Encapsulates data for exiting the program due to an error error
#[derive(Debug, PartialEq)]
pub struct DriverError<'a> {
    /// Exit code to exit the program with, due to the error
    pub exit_code: i32,
    /// Error message
    pub msg: Cow<'a, str>,
}
