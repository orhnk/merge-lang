use crate::Token;

pub struct Parser {
    input: Vec<Token>, // The input tokens
    current: usize,    // Index of the current token
}

impl Parser {
    // TODO: Add new and focus methods. (focus to change the content and reset the cursor)
    // Constructor for the Parser struct.
    pub fn from(input: Vec<Token>) -> Self {
        Parser {
            input,
            current: 0,
        }
    }

    // Entry point for the parser. Parses and evaluates an expression.
    pub fn parse_expression(&mut self) -> f64 {
        self.parse_addition()
    }

    // Parses addition and subtraction.
    pub fn parse_addition(&mut self) -> f64 {
        let mut result = self.parse_multiplication();

        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.consume(); // Consume the '+' token
                    result += self.parse_multiplication();
                }
                Token::Minus => {
                    self.consume(); // Consume the '-' token
                    result -= self.parse_multiplication();
                }
                _ => break,
            }
        }

        result
    }

    // Parses multiplication and division.
    pub fn parse_multiplication(&mut self) -> f64 {
        let mut result = self.parse_primary();

        while let Some(token) = self.peek() {
            match token {
                Token::Multiply => {
                    self.consume(); // Consume the '*' token
                    result *= self.parse_primary();
                }
                Token::Divide => {
                    self.consume(); // Consume the '/' token
                    let denominator = self.parse_primary();
                    if denominator != 0.0 {
                        result /= denominator;
                    } else {
                        panic!("Division by zero");
                    }
                }
                _ => break,
            }
        }

        result
    }

    // Parses primary expressions (numbers and parentheses).
    pub fn parse_primary(&mut self) -> f64 {
        if let Some(token) = self.peek() {
            match token {
                #[allow(unused_mut)]
                Token::Number(mut num) => {
                    // HERE HELP REMOVE THIS `MUT`
                    self.consume(); // Consume the number token
                    num
                }
                Token::LeftParenthesis => {
                    self.consume(); // Consume the '(' token
                    let result = self.parse_addition();
                    if let Some(token) = self.peek() {
                        if *token == Token::RightParenthesis {
                            self.consume(); // Consume the ')' token
                            return result;
                        }
                    }
                    panic!("Unmatched '('");
                }
                _ => panic!("Unexpected token"),
            }
        } else {
            panic!("Unexpected end of input");
        }
    }

    // Helper function to peek at the current token.
    pub fn peek(&self) -> Option<&Token> {
        if self.current < self.input.len() {
            Some(&self.input[self.current])
        } else {
            None
        }
    }

    // Helper function to consume the current token.
    pub fn consume(&mut self) {
        if self.current < self.input.len() {
            self.current += 1;
        }
    }
}

