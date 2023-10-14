//! # Merge Meta Programming Language
//!
//! A generic meta programming language that
//! automates multi-lang communication protocols
//!
//! ## TODO
//!
//! ### Fix Syntax Issues
//!
//! `[vars] -> returns` is a bad way to name returning parameters.
//!
//! Firstly It is harder to know which variable is getting returned.
//! And that is what merge is! So think about a better solution ASAP!
//!
//! ## Philosophy
//!
//! Writing projects with multiple programming languages can easily become a hard task.
//! With Merge, you have the option to automatically generate appropriate protocols in to your code
//! With an abstracted interface that doesn't require any knowladge about what is under the hood.
//!
//! ## Desired Syntax
//!
//! One of the important portion of a programming language to the end user is syntax design
//! Merge uses a homely syntax that is non-ambigious, sensible, simple
//! ("Code should be easy to read AND write")
//!
//! Here is the general Merge code you'll see across the programs:
//!
//! ```no_run
//! !{
//!     rust = {
//!         build = "rust/Cargo.toml",  // Default
//!         cmd   = "cargo run",        // Default
//!         test  = "cargo test",       // Default
//!         release = {
//!             cmd = "cargo run --release" // Default
//!             // <OTHER OPTS>
//!         },
//!     },
//!     c = {
//!         build = "c/CMakeLists.txt", // Default
//!         cmd   = "cmake -B build/",  // Default
//!         test  = "make test",        // Default
//!         release = {
//!             cmd = "cmake -B build -DRELEASE=true" // Default
//!             // <OTHER OPTS>
//!         },
//!     },
//! }
//!
//! -> [ // Scoop these varaibles to merge
//!     first,
//!     second,
//!     third
//! ]    // _.-' FROM HERE '-._
//! rust! {
//!     let mut first  = 2; // Inentionally Wrong
//!     let second = 2;
//!     let third  = 3;
//!     
//!     println!("[RUST]: Hello, World!");
//!     
//!     first = 1; // Overwritten
//!     // implicitly returns the variables
//! }
//!
//! //_.-' DUMP TO C '-._      _.-' SCOOP '-._
//! [first, second, third] -> [scooped_from_c]
//! c! {
//!     printf("[C]: STARTED");
//!     printf("%d, %d, %d", first, second, third); // 1, 2, 3
//!     int scooped_from_c = 1;
//!     printf("[C]: ENDED");
//! }
//!
//! ```
//!
//!
//!
//!
//!
//! ## Cons
//!
//! You will probably need to know a lot when it comes to types.
//! What do I mean?
//! Because Merge is so generic, you'll need to manage your types manually
//! Which is expected because of the philosophical difference between languages.
//! (e.g) Python doesn't have types while Rust strictly does.

#![allow(warnings, unused)]

/// Converts String Literal into inline! token matcher
/// returns: Regex Patterns
macro_rules! keyword_regex {
    ($lit:literal) => {{
        // (?s)\s*python!\((.*?)\)
        concat!(r"(?s)\s*", $lit, r"!\((.*?)\)")
    }};
}

#[allow(unused)]
enum InLang {
    C,
    CSharp,
    Cpp,
    Cobol,
    Carbon,
    D,
    Go,
    Haskell,
    Merge,
    OCaml,
    Python,
    R,
    Rust,
    Racket,
    V,
}

impl InLang {
    // TODO: this is ambigious by the case size (upper - lower)
    fn from(lang: &str) -> Option<Self> {
        return match lang {
            // TODO: Compile time manipulate this to lowercase
            "c" => Some(Self::C),
            "csharp" => Some(Self::CSharp),
            "cpp" => Some(Self::Cpp),
            "cobol" => Some(Self::Cobol),
            "carbon" => Some(Self::Carbon),
            "d" => Some(Self::D),
            "go" => Some(Self::Go),
            "haskell" => Some(Self::Haskell),
            "merge" => Some(Self::Merge),
            "ocaml" => Some(Self::OCaml),
            "python" => Some(Self::Python),
            "r" => Some(Self::R),
            "rust" => Some(Self::Rust),
            "racket" => Some(Self::Racket),
            "v" => Some(Self::V),
            _ => None,
        };
    }
}

use std::cmp::PartialEq;

// Define the token type for our parser.
#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
enum Token {
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

// Define a struct to represent the parser.
struct Parser {
    input: Vec<Token>, // The input tokens
    current: usize,    // Index of the current token
}

impl Parser {
    // TODO: Add new and focus methods. (focus to change the content and reset the cursor)
    // Constructor for the Parser struct.
    fn from(input: Vec<Token>) -> Self {
        Parser { input, current: 0 }
    }

    // Helper function to peek at the current token.
    fn peek(&self) -> Option<&Token> {
        if self.current < self.input.len() {
            Some(&self.input[self.current])
        } else {
            None
        }
    }

    // Helper function to consume the current token.
    fn consume(&mut self) {
        if self.current < self.input.len() {
            self.current += 1;
        }
    }
}

// Define a struct to represent the lexer.
#[derive(Debug, Eq, PartialEq)]
struct Lexer {
    input: String,   // The input string
    position: usize, // Current position in the input string
}

#[allow(dead_code)]
impl Lexer {
    // Constructor for the Lexer struct.
    fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    // Helper function to advance the lexer's position.
    fn advance(&mut self) {
        self.position += 1;
    }

    fn get(&self, index: usize) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input.chars().nth(index).unwrap() // for the sake of lazy evaluation
    }

    fn pos(&self) -> usize {
        self.position
    }

    // Helper function to skip whitespace characters.
    fn skip_whitespace(&mut self) {
        // TODO: rename this function according to whitespaces_and_comments which I'm too lazy to do now.
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' && self.get(self.pos() + 1usize) == '/' {
            } else {
                break;
            }
        }
    }

    // Helper function to peek at the current character.
    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input.chars().nth(self.position).unwrap()) // TODO: use unicode-segmentation
        } else {
            None
        }
    }

    // Helper function to consume the current character.
    fn consume(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            self.advance();
            Some(c)
        } else {
            None
        }
    }

    // Tokenizes the input string.
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        self.skip_whitespace(); // Skip initial whitespaces

        while let Some(c) = self.peek() {
            tokens.push(self.check_all());
            self.skip_whitespace();
        }

        tokens.push(Token::EndOfFile);
        tokens
    }

    #[inline]
    pub fn check_all(&mut self) -> Token {
        // TODO: check whether is it necessary to de-advance this implicit self.advance() call
        self.match_char_token(self.position)
    }

    fn match_key_token(&mut self, index: usize) -> Option<Token> {
        todo!()
    }

    fn consume_identifier_char(&mut self) -> Token {
        let c = self.input.chars().nth(self.position).unwrap();
        if c.is_alphabetic() || c == '_' {
            return Token::Identifier(c.to_string()); // FIXME: With better API
        }
        return Token::Invalid(c.to_string()); // FIXME: With better API
    }

    /// Returns identifier under the cursor.
    /// Returns [`None`] if the identifier is not there
    fn consume_identifier(&mut self, index: usize) -> Option<Token> {
        let c = self.input.chars().nth(index).unwrap();
        let mut identifier = String::new();

        while let Token::Identifier(ch) = self.consume_identifier_char() {
            identifier.push_str(&ch); // FIXME: these are just chars that are pushed
            self.advance();
        }

        if identifier.is_empty() {
            None
        } else {
            Some(Token::Identifier(identifier))
        }
    }

    fn match_char_token(&mut self, index: usize) -> Token {
        let c = self.input.chars().nth(index).unwrap();
        match self.input.chars().nth(index).unwrap() {
            '!' => {
                // macro call
                // Or a code block
                self.advance();
                self.skip_whitespace();
                if let Some('{') = self.peek() {
                    // Code block
                    self.advance();
                    let mut code = String::new();
                    while let Some(ch) = self.peek() {
                        if ch == '}' {
                            self.advance();
                            break;
                        } else {
                            code.push(ch);
                            self.advance();
                        }
                    }
                    return Token::Code(code);
                } else {
                    // Macro call
                    Token::ExclamationMark
                }
            }
            '[' => {
                self.advance();
                Token::LeftSquareBracket
            }
            ']' => {
                self.advance();
                Token::RightSquareBracket
            }
            '{' => {
                self.advance();
                Token::LeftCurlyBracket
            }
            '}' => {
                self.advance();
                Token::RightCurlyBracket
            }
            '=' => {
                self.advance();
                Token::EqualSign
            }
            '-' => {
                self.advance(); // TODO Add these as keywords not char tokens
                if let Some('>') = self.peek() {
                    self.advance();
                    Token::Arrow
                } else {
                    Token::Invalid(c.to_string())
                }
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            '"' => {
                self.advance();
                Token::Quote
            }
            '#' => {
                self.advance();
                Token::Hashtag
            }
            _ => {
                // Mapped to an Identifier
                let identifier_or_invalid = self.consume_identifier(self.position);
                if let Some(Token::Identifier(ref identifier)) = identifier_or_invalid {
                    return identifier_or_invalid.unwrap();
                } else {
                    self.advance(); // TODO take these as keyword
                    return Token::Invalid(c.to_string());
                }
                // if let Some(key_token) = self.match_key_token(self.position) {
                //     // TODO
                // }

                // NOTE: this is the _ arm of the match block which means that
                // every possible token check is done! So we can create an
                // identifier from the most enhanced cluster (emojis, + signs etc.)

                // Identifier check
                let mut identifier = String::new();

                while let Some(ch) = self.input.chars().nth(self.position) {
                    if ch.is_alphabetic() || ch == '_' {
                        identifier.push(ch);
                        self.advance();
                    } else {
                        self.advance();
                        return Token::Invalid(ch.to_string()); // TODO: Fix these dumb .to_string() calls
                    }
                    self.skip_whitespace();
                }
                return Token::Identifier(identifier);
            }
        }
    }

    //    #[inline]
    //    pub fn check_block(&mut self, c: char, s: &str) -> Token {
    //                    match c {
    //                '+' => {
    //                    self.advance();
    //                    tokens.push(Token::Plus);
    //                }
    //                '-' => {
    //                    self.advance();
    //                    tokens.push(Token::Minus);
    //                }
    //                '*' => {
    //                    self.advance();
    //                    tokens.push(Token::Multiply);
    //                }
    //                '/' => {
    //                    self.advance();
    //                    tokens.push(Token::Divide);
    //                }
    //                '(' => {
    //                    self.advance();
    //                    tokens.push(Token::LeftParenthesis);
    //                }
    //                ')' => {
    //                    self.advance();
    //                    tokens.push(Token::RightParenthesis);
    //                }
    //                '0'..='9' => {
    //                    let mut num_str = String::new();
    //                    while let Some(digit) = self.peek() {
    //                        if digit.is_digit(10) || digit == '.' {
    //                            num_str.push(digit);
    //                            self.advance();
    //                        } else {
    //                            break;
    //                        }
    //                    }
    //                    if let Ok(num) = num_str.parse::<f64>() {
    //                        tokens.push(Token::Number(num));
    //                    } else {
    //                        tokens.push(Token::Invalid(c));
    //                    }
    //                }
    //                _ => {
    //                    self.advance(); // Skip invalid characters
    //                    tokens.push(Token::Invalid(c));
    //                }
    //            }
    //    }
    //
    //    #[inline]
    //    pub fn parse_delimeter(&mut self, c: char) -> Token {
    //        match c {
    //            '+' => {
    //                self.advance();
    //                tokens.push(Token::Plus);
    //            }
    //            '-' => {
    //                self.advance();
    //                tokens.push(Token::Minus);
    //            }
    //            '*' => {
    //                self.advance();
    //                tokens.push(Token::Multiply);
    //            }
    //            '/' => {
    //                self.advance();
    //                tokens.push(Token::Divide);
    //            }
    //            '(' => {
    //                self.advance();
    //                tokens.push(Token::LeftParenthesis);
    //            }
    //            ')' => {
    //                self.advance();
    //                tokens.push(Token::RightParenthesis);
    //            }
    //            '0'..='9' => {
    //                let mut num_str = String::new();
    //                while let Some(digit) = self.peek() {
    //                    if digit.is_digit(10) || digit == '.' {
    //                        num_str.push(digit);
    //                        self.advance();
    //                    } else {
    //                        break;
    //                    }
    //                }
    //                if let Ok(num) = num_str.parse::<f64>() {
    //                    tokens.push(Token::Number(num));
    //                } else {
    //                    tokens.push(Token::Invalid(c));
    //                }
    //            }
    //            _ => {
    //                self.advance(); // Skip invalid characters
    //                tokens.push(Token::Invalid(c));
    //            }
    //        }
    //    }
}

fn main() {
    let rstr = r#"
![
    rust = {
        build = "hello world",
    },
]

#[main]
[one, two, three] -> [scooped]
lang! {
    println!("It's A me! mArio!");
}
"#
    .to_string();

    let mut lexer = Lexer::new(rstr);
    let mut invalid_buf = Vec::new();
    let tokens = lexer.tokenize();
    tokens.iter().for_each(|i| {
        if let Token::Invalid(ch) = i {
            invalid_buf.push(ch);
        }
    });

    println!("{tokens:#?}");

    let mut parser = Parser::from(tokens);
}
