use crate::Token;

use super::error::ParserError;

pub struct Parser {
    tokens: Vec<Token>, // The input tokens
    cursor: usize,      // Index of the current token
}

impl Parser {
    // todo: add new and focus methods. (focus to change the content and reset the cursor)
    // Constructor for the Parser struct.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    // Entry point for the parser. Parses and evaluates an expression.
    pub fn parse(&mut self) -> Result<(), Vec<ParserError>> {
        self.parse_macro()
        // ![
        //      <CONFIG OPTS>
        // ]
    }

    pub fn parse_macro(&mut self) -> Result<(), Vec<ParserError>> {
        if let Some(Token::Macro(_)) = self.peek() {
            self.advance();
            self.parse_config_opts()?;
            Ok(())
        } else {
            Err(vec![ParserError::UnexpectedToken(Token::ExclamationMark)])
        }
    }

    /// Parses the config options.
    /// Given above in the example, this would be:
    ///
    /// ![
    ///     rust = {
    ///        build = "hello world",
    ///     },
    /// ]
    pub fn parse_config_opts(&mut self) -> Result<(), Vec<ParserError>> {
        // ![
        //     rust = {
        //         build = "hello world",
        //     },
        // ]

        let mut curly_bracket_count = Vec::<Token>::new();
        let mut square_bracket_coun = Vec::<Token>::new();

        let cursor_token = self.peek();

        println!("Cursor Token: {:?}", cursor_token);

        match cursor_token {
            Some(Token::LeftSquareBracket) => {
                square_bracket_coun.push(Token::LeftSquareBracket);
                self.advance();
                self.parse_config_opts()?;
                Ok(())
            }

            Some(Token::RightSquareBracket) => {
                if square_bracket_coun.len() > 0 {
                    square_bracket_coun.pop();
                } else {
                    return Err(vec![ParserError::UnexpectedToken({
                        cursor_token.expect("Unexpected Token").clone() // FIXME: clone
                    })]);
                }
                self.advance();
                Ok(())
            }

            Some(Token::Identifier(_)) => {
                self.parse_config_opts()?;
                Ok(())
            }

            Some(Token::String(s)) => {
                println!("Config String: {}", s);
                self.parse_config_opts()?;
                Ok(())
            }

            Some(Token::EqualSign) => {
                self.advance();
                self.parse_config_opts()?;
                Ok(())
            }

            Some(Token::LeftCurlyBracket) => {
                self.advance();
                self.parse_config_opts()?;
                Ok(())
            }

            Some(Token::RightCurlyBracket) => {
                self.advance();
                self.parse_config_opts()?;
                Ok(())
            }

            // Enforce comma separation
            Some(Token::Comma) => {
                self.advance();
                self.parse_config_opts()?;
                Ok(())
            }

            Some(Token::Identifier(_)) => {
                self.advance();
                self.parse_code()?;
                Ok(())
            }

            Some(Token::Macro(_)) => {
                self.advance();
                self.parse_code()?;
                Ok(())
            }

            _ => {
                println!(
                    "Unexpected Token: {:?}",
                    cursor_token.expect("Unexpected Token")
                );
                Err(vec![ParserError::UnexpectedToken({
                    cursor_token.expect("Unexpected Token").clone() // FIXME: clone
                })])
            }
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.cursor < self.tokens.len() {
            Some(&self.tokens[self.cursor])
        } else {
            None
        }
    }

    // Helper function to consume the current token.
    pub fn advance(&mut self) {
        if self.cursor < self.tokens.len() {
            self.cursor += 1;
        }
    }
}
