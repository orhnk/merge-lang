#[test]
fn test_tokenizer() {
    let rstr = r###"
            ![
                rust = {
                    build = "hello world",
                },
            ]

            # hello
            ![main]
            [one, two, three] -> [scooped]
            rust! {
                println!("It's A me! mArio!");
                let scooped = "RUST STRING";
                {
                    println!("inside block");
                }
            }

            [scooped]
            c! {
                printf("Hello, World!");
                {},
                printf("%s", scooped);
                char* scooped = "C STRING";
            }

        "###
    .to_string();

    let mut lexer = Tokenizer::new(rstr.into());

    let tokens = lexer.tokenize();
    tokens.iter().for_each(|i| {
        if let Token::Invalid(ch) = i {
            panic!("Invalid token: {}", ch);
        }
    });
}
