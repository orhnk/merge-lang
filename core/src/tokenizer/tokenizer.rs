use crate::Token;
use bytestring::ByteString;

// Define a struct to represent the lexer.
#[derive(Debug, Eq, PartialEq)]
pub struct Tokenizer {
    input: ByteString, // The input string
    position: usize,   // Current position in the input string
}

#[allow(dead_code)]
impl Tokenizer {
    // Constructor for the Lexer struct.
    pub fn new(input: ByteString) -> Self {
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
                    let mut curly_bracket_count = 1usize;
                    while let Some(ch) = self.peek() {
                        // Match the code block
                        if ch == '{' {
                            curly_bracket_count += 1;
                        } else if ch == '}' {
                            curly_bracket_count -= 1;
                        }
                        if curly_bracket_count == 0 {
                            self.advance();
                            break;
                        } else {
                            code.push(ch);
                            self.advance();
                        }
                    }
                    Token::Code(code)
                } else if let Some('[') = self.peek() {
                    // Macro call
                    self.advance();
                    let mut macro_call = String::new();
                    let mut square_bracket_count = 1usize;
                    while let Some(ch) = self.peek() {
                        // Match the code block
                        if ch == '[' {
                            square_bracket_count += 1;
                        } else if ch == ']' {
                            square_bracket_count -= 1;
                        }
                        if square_bracket_count == 0 {
                            self.advance();
                            break;
                        } else {
                            macro_call.push(ch);
                            self.advance();
                        }
                    }
                    Token::Macro(macro_call)
                } else {
                    Token::ExclamationMark
                }
            }
            '[' => {
                // [comma, separated, variables] -> [comma, separated, variables]
                self.advance();

                let mut take_variables = Vec::<String>::new();
                let mut send_var_buf = String::new();

                while let Some(ch) = self.peek() {
                    if ch == ']' {
                        self.advance();
                        if !send_var_buf.is_empty() {
                            take_variables.push(send_var_buf);
                        }
                        // else {
                        //     Token::Invalid(ch.to_string()) // Trailing comma
                        // }
                        break;
                    } else if ch == ',' {
                        take_variables.push(send_var_buf.clone());
                        self.advance();
                    } else {
                        send_var_buf.push(ch);
                        self.advance();
                    }
                }

                self.skip_whitespace();

                if self.rush("->") {
                    self.advance();
                    self.skip_whitespace();

                    let mut send_variables = Vec::<String>::new();
                    let mut send_var_buf = String::new();

                    while let Some(ch) = self.peek() {
                        if ch == ']' {
                            self.advance();
                            if !send_var_buf.is_empty() {
                                send_variables.push(send_var_buf);
                            }
                            // else {
                            //     Token::Invalid(ch.to_string()) // Trailing comma
                            // }
                            break;
                        } else if ch == ',' {
                            send_variables.push(send_var_buf.clone());
                            self.advance();
                        } else {
                            send_var_buf.push(ch);
                            self.advance();
                        }
                    }

                    return Token::Bridge {
                        take: Some(take_variables),
                        send: Some(send_variables),
                    };
                }

                Token::Bridge {
                    take: Some(take_variables),
                    send: None,
                }
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
                    self.skip_whitespace();
                    // -> [ <VARIALBES> ]
                    //    ^^^^^^^^^^^^^^^-- Arrow by itself is invalid.
                    //    |---------------- Variables are seperated by commas.
                    let mut variables = Vec::<String>::new();
                    let mut var_buf = String::new();

                    if let Some('[') = self.peek() {
                        self.advance();
                        while let Some(ch) = self.peek() {
                            if ch == ']' {
                                self.advance();
                                if !var_buf.is_empty() {
                                    variables.push(var_buf);
                                }
                                // else {
                                //     Token::Invalid(ch.to_string()) // Trailing comma
                                // }
                                break;
                            } else if ch == ',' {
                                variables.push(var_buf.clone()); // FIXME: clone <09-11-23, utfeight>
                                self.advance();
                            } else {
                                var_buf.push(ch);
                                self.advance();
                            }
                        }
                    }
                    Token::Bridge {
                        take: None,
                        send: Some(variables),
                    }
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
            }
        }
    }

    /// Checks whether the string is present at the current position.
    /// Moves the cursor to the end of the string if it is present.
    /// Does not move the cursor if the string is not present.
    /// Returns `true` if the string is present.
    /// Returns `false` if the string is not present.
    // TODO: CHECK THIS OUT
    pub fn rush(&mut self, string: &str) -> bool {
        if self.input[self.position..].starts_with(string) {
            self.position += string.len();
            true
        } else {
            false
        }
    }
}
