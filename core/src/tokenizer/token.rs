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
    /// [
    LeftSquareBracket,
    /// ]
    RightSquareBracket,
    /// (
    LeftRoundBracket,
    /// )
    RightBracket,
    /// =
    EqualSign,
    /// "<STRING>"
    String(String),
    /// ,
    Comma,
    /// #
    Comment(String),
    /// ->
    Arrow,
    /// <lang>! { <CODE> }
    Code(String),
    /// ![ <MACRO> ]
    Macro(String),
    /// [one, two, three] -> [four, five, six]
    Bridge {
        take: Option<Vec<String>>,
        send: Option<Vec<String>>,
    },
    EndOfFile,
    Identifier(String),
    Invalid(String),
}
