use crate::Token;
// Define a struct to represent the lexer.
#[derive(Debug, Eq, PartialEq)]
pub struct Tokenizer {
    input: String,   // The input string
    position: usize, // Current position in the input string
}

#[allow(dead_code)]
impl Tokenizer {
    // Constructor for the Lexer struct.
    pub fn new(input: String) -> Self {
        Tokenizer { input, position: 0 }
    }

    // Helper function to advance the lexer's position.
    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn get(&self, index: usize) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input.chars().nth(index).unwrap() // for the sake of lazy evaluation
    }

    pub fn pos(&self) -> usize {
        self.position
    }

    // Helper function to skip whitespace characters.
    pub fn skip_whitespace(&mut self) {
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

        while let Some(_c) = self.peek() {
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

    pub fn match_key_token(&mut self, _index: usize) -> Option<Token> {
        todo!()
    }

    pub fn consume_identifier_char(&mut self) -> Token {
        let c = self.input.chars().nth(self.position).unwrap();
        self.advance();
        if c.is_alphabetic() || c == '_' {
            return Token::Identifier(c.to_string()); // FIXME: With better API
        }
        Token::Invalid(c.to_string()) // FIXME: With better API
    }

    pub fn peek_identifier_char(&mut self) -> Token {
        let c = self.input.chars().nth(self.position).unwrap();
        if c.is_alphabetic() || c == '_' {
            return Token::Identifier(c.to_string()); // FIXME: With better API
        }
        Token::Invalid(c.to_string()) // FIXME: With better API
    }

    /// Returns identifier under the cursor.
    /// Returns [`None`] if the identifier is not there
    pub fn consume_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while let Token::Identifier(ch) = self.peek_identifier_char() {
            identifier.push_str(&ch); // FIXME: these are just chars that are pushed
            self.advance();
        }

        if identifier.is_empty() {
            self.consume_identifier_char()
        } else {
            Token::Identifier(identifier)
        }
    }

    pub fn match_char_token(&mut self, index: usize) -> Token {
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
                    Token::Code(code)
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
            '#' => {
                self.advance(); // TODO Add these as keywords not char tokens
                let mut comment = String::new();

                while let Some(ch) = self.peek() {
                    if ch == '\n' {
                        self.advance();
                        break;
                    } else {
                        comment.push(ch);
                        self.advance();
                    }
                }

                Token::Comment(comment)
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
                let mut string = String::new();
                while let Some(ch) = self.peek() {
                    if ch == '"' {
                        self.advance();
                        break;
                    } else {
                        string.push(ch);
                        self.advance();
                    }
                }
                Token::String(string)
            }
            _ => {
                // Mapped to an Identifier
                self.consume_identifier()

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

