use crate::tokenizer::Token;

#[derive(Debug)]
pub struct Preprocessor<'a> {
    tokens: &'a mut Vec<Token>,
    opts: PreprocessorOpts,
}

#[derive(Debug)]
pub struct PreprocessorOpts {
    comments: bool,
}

impl Default for PreprocessorOpts {
    fn default() -> Self {
        Self { comments: true }
    }
}

impl<'a> Preprocessor<'a> {
    pub fn new(tokens: &'a mut Vec<Token>, opts: PreprocessorOpts) -> Self {
        Self { tokens, opts }
    }

    /// Preprocess the tokens
    ///
    /// # Logic
    ///
    /// - Remove comments
    pub fn preprocess(&mut self) {
        self.remove_comments()
    }

    pub fn remove_comments(&mut self) {
        let tokens = &mut self.tokens;
        let mut i = 0;

        while i < tokens.len() {
            if let Token::Comment(_) = tokens[i] {
                tokens.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
