mod log;
mod parser;
mod prep;
mod tokenizer;
mod inline;
mod task;

use crate::tokenizer::{Tokenizer, Token};

fn main() {
    let rstr = r#"
![
    rust = {
        build = "hello world",
    },
]

#[main]
[one, two, three] -> [scooped]
rust! {
    println!("It's A me! mArio!");
    let scooped = "RUST STRING"
}

[scooped]
c! {
    printf("Hello, World!");
    printf("%s", scooped);
    char* scooped = "C STRING";
}
"#
    .to_string();

    let mut lexer = Tokenizer::new(rstr);
    let mut invalid_buf = Vec::new();

    let tokens = lexer.tokenize();
    tokens.iter().for_each(|i| {
        if let Token::Invalid(ch) = i {
            invalid_buf.push(ch);
        }
    });

    println!("{tokens:#?}");

    // let mut parser = Parser::from(tokens);
}

