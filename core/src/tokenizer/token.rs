// Define the token type for our parser.
#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum Token {
    ExclamationMark,    // !
    LeftCurlyBracket,   // {
    RightCurlyBracket,  // }
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    LeftRoundBracket,   // (
    RightBracket,       // )
    EqualSign,          // =
    Quote,              // "
    Comma,              // ,
    Hashtag,            // #
    Arrow,              // ->
    Code(String),       // TODO: Anything between `<lang>! { <CODE> }` braces
    EndOfFile,
    Identifier(String),
    Invalid(String),
}

