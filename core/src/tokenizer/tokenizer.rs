use crate::tokenizer::Token;
// Define a struct to represent the parser.
// Define a struct to represent the tokenizer.
#[derive(Debug, Eq, PartialEq)]
pub struct Tokenizer {
    input: String,   // The input string
    position: usize, // Current position in the input string
}

#[allow(dead_code)]
impl Tokenizer {
    // Constructor for the tokenizer struct.
    pub fn new(input: String) -> Self {
        Tokenizer { input, position: 0 }
    }

    /// Moves the cursor to the next character.
    pub fn advance(&mut self) {
        self.position += 1;
    }

    /// Getter for the current character.
    pub fn get_cursor_char(&self) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input.chars().nth(self.pos()).unwrap() // for the sake of lazy evaluation
    }

    /// Offset the cursor by `index` and return the character.
    pub fn offset(&self, index: isize) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input
            .chars()
            .nth(
                (TryInto::<isize>::try_into(self.pos()).expect("usize & isize Overflow") as isize
                    + index) as usize,
            )
            .unwrap()
    }

    /// Get the character at `index`.
    pub fn get_exact(&self, index: usize) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input.chars().nth(index).unwrap() // for the sake of lazy evaluation
    }

    /// Getter for the cursor position.
    pub fn pos(&self) -> usize {
        self.position
    }

    /// Helper function to skip whitespace characters.
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Helper function to peek at the current character.
    pub fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input.chars().nth(self.position).unwrap()) // TODO: use unicode-segmentation
        } else {
            None
        }
    }

    /// Helper function to consume the current character.
    pub fn consume(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            self.advance();
            Some(c)
        } else {
            None
        }
    }

    /// Tokenizes the input string.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        self.skip_whitespace(); // Skip initial whitespaces

        while let Some(c) = self.peek() {
            match c {
                '+' => {
                    self.advance();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    self.advance();
                    tokens.push(Token::Multiply);
                }
                '/' => {
                    self.advance();
                    tokens.push(Token::Divide);
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::LeftParenthesis);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::RightParenthesis);
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    while let Some(digit) = self.peek() {
                        if digit.is_digit(10) || digit == '.' {
                            num_str.push(digit);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if let Ok(num) = num_str.parse::<f64>() {
                        tokens.push(Token::Number(num));
                    } else {
                        tokens.push(Token::Invalid(c));
                    }
                }
                _ => {
                    self.advance(); // Skip invalid characters
                    tokens.push(Token::Invalid(c));
                }
            }
            self.skip_whitespace();
        }

        tokens.push(Token::EndOfFile);
        tokens
    }
}

// struct TokenizerOpts {
//     inline: bool,
// }
//
// impl std::default::Default for TokenizerOpts {
//     TokenizerOpts {
//         inline: false,
//     }
// }
