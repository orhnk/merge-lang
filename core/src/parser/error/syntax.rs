use thiserror::Error;

// Lexer + Parser Error
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Expected: {0}, found: {1}")]
    IncompleteToken(String, String),
}

