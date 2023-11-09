// Define the token type for our parser.
#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum Token {
    /// !
    ExclamationMark,
    /// {
    LeftCurlyBracket,
    /// }
    RightCurlyBracket,
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    LeftRoundBracket,   // (
    RightBracket,       // )
    EqualSign,          // =
    String(String),     // "<CONTENT>"
    Comma,              // ,
    Comment(String),    // #
    Arrow,              // ->
    Code(String),       // TODO: Anything between `<lang>! { <CODE> }` braces
    LeftSquareBracket,
    RightSquareBracket,
    LeftRoundBracket,
    RightBracket,
    EqualSign,
    String(String),
    Comma,
    Comment(String),
    Arrow,
    Code(String),
    Macro(String),
    Bridge {
        take: Option<Vec<String>>,
        send: Option<Vec<String>>,
    },
    EndOfFile,
    Identifier(String),
    Invalid(String),
}
