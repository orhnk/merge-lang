//! Compilation errors for merge compiler
//!
//! This module contains the utililty for handling compilation errors in a prettier way.
//! checks for [`InvalidToken`] and tries to find the closest valid token to the invalid one.
//! (logically, not syntactically)

use crate::parser::error::*; // Parser Errors (e.g SyntaxError)
use crate::tokenizer::Token::Invalid; // InvalidToken variant of Token enum
