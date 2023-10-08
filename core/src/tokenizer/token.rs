// Define the token type for our parser.
#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    EndOfFile,
    Invalid(char),
}

