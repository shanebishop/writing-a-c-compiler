use std::io;

/// Encapsulates data for exiting the program due to an error error.
///
/// We cannot derive any `From` implementations using `thiserror::Error`,
/// because main always expects an exit code to be able to exit with.
/// Therefore, all `From` implementations are handwritten.
#[derive(Debug, PartialEq)]
pub struct DriverError {
    /// Exit code to exit the program with, due to the error
    pub exit_code: i32,
    /// Error message
    pub msg: String,
}

impl From<io::Error> for DriverError {
    fn from(e: std::io::Error) -> Self {
        Self {
            exit_code: 1,
            msg: format!("I/O error: {e}"),
        }
    }
}
