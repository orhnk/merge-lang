use thiserror::Error;

use crate::tokenizer::Token;

// Lexer + Parser Error
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Expected: {0}, found: {1}")]
    IncompleteToken(String, String),
}

#[derive(Debug)]
pub enum ParserError {
    Syntax(SyntaxError),
    UnexpectedToken(Token),
}
