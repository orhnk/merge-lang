use crate::tokenizer::Token;
// Define a struct to represent the parser.
// Define a struct to represent the lexer.
#[derive(Debug, Eq, PartialEq)]
pub struct Lexer {
    input: String,   // The input string
    position: usize, // Current position in the input string
}

#[allow(dead_code)]
impl Lexer {
    // Constructor for the Lexer struct.
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    // Helper function to advance the lexer's position.
    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn get(&self) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input.chars().nth(self.pos()).unwrap() // for the sake of lazy evaluation
    }

    pub fn index(&self, index: isize) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input
            .chars()
            .nth((TryInto::<isize>::try_into(self.pos()).expect("usize & isize Overflow") as isize + index) as usize)
            .unwrap()
    }

    pub fn get_exact(&self, index: usize) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input.chars().nth(index).unwrap() // for the sake of lazy evaluation
    }

    pub fn pos(&self) -> usize {
        self.position
    }

    pub fn check(&self, lit: &str) -> bool {
        let mut index = 0isize;
        let mut lit_chars = lit.chars();
        for c in lit_chars {
            if c != self.index(index) {
                return false;
            }
            index += 1;
        }
        true
    }

    // Helper function to skip whitespace characters.
    pub fn skip_whitespace(&mut self) {
        // TODO: rename this function according to whitespaces_and_comments which I'm too lazy to do now.
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' && self.get() == '/' {
            } else {
                break;
            }
        }
    }

    // Helper function to peek at the current character.
    pub fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input.chars().nth(self.position).unwrap()) // TODO: use unicode-segmentation
        } else {
            None
        }
    }

    // Helper function to consume the current character.
    pub fn consume(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            self.advance();
            Some(c)
        } else {
            None
        }
    }

    // Tokenizes the input string.
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

