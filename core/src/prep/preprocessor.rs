use crate::tokenizer::Token;

static COMMENT_STR: &str = "//";

#[derive(Debug)]
pub struct Preprocessor {
    tokens: Vec<Token>,
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

impl Preprocessor {
    pub fn new(tokens: Vec<Token>, opts: PreprocessorOpts) -> Self {
        Self { tokens, opts }
    }

    pub fn preprocess(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut comment = false;

        for token in self.tokens.iter() {
            match token {
                Token::Comment(_) => {
                    if self.opts.comments {
                        comment = true;
                    }
                }
                Token::EndOfFile => {
                    comment = false;
                }
                _ => {
                    if !comment {
                        tokens.push(token.clone());
                    }
                }
            }
        }

        tokens
    }
}
