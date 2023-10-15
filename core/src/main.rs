//! # Merge Meta Programming Language
//!
//! A generic meta programming language that
//! automates multi-lang communication protocols
//!
//! ## TODO
//!
//! ### Fix Syntax Issues
//!
//! `[vars] -> returns` is a bad way to name returning parameters.
//!
//! Firstly It is harder to know which variable is getting returned.
//! And that is what merge is! So think about a better solution ASAP!
//!
//! ## Philosophy
//!
//! Writing projects with multiple programming languages can easily become a hard task.
//! With Merge, you have the option to automatically generate appropriate protocols in to your code
//! With an abstracted interface that doesn't require any knowladge about what is under the hood.
//!
//! ## Desired Syntax
//!
//! One of the important portion of a programming language to the end user is syntax design
//! Merge uses a homely syntax that is non-ambigious, sensible, simple
//! ("Code should be easy to read AND write")
//!
//! Here is the general Merge code you'll see across the programs:
//!
//! ```no_run
//! !{
//!     rust = {
//!         build = "rust/Cargo.toml",  // Default
//!         cmd   = "cargo run",        // Default
//!         test  = "cargo test",       // Default
//!         release = {
//!             cmd = "cargo run --release" // Default
//!             // <OTHER OPTS>
//!         },
//!     },
//!     c = {
//!         build = "c/CMakeLists.txt", // Default
//!         cmd   = "cmake -B build/",  // Default
//!         test  = "make test",        // Default
//!         release = {
//!             cmd = "cmake -B build -DRELEASE=true" // Default
//!             // <OTHER OPTS>
//!         },
//!     },
//! }
//!
//! -> [ // Scoop these varaibles to merge
//!     first,
//!     second,
//!     third
//! ]    // _.-' FROM HERE '-._
//! rust! {
//!     let mut first  = 2; // Inentionally Wrong
//!     let second = 2;
//!     let third  = 3;
//!     
//!     println!("[RUST]: Hello, World!");
//!     
//!     first = 1; // Overwritten
//!     // implicitly returns the variables
//! }
//!
//! //_.-' DUMP TO C '-._      _.-' SCOOP '-._
//! [first, second, third] -> [scooped_from_c]
//! c! {
//!     printf("[C]: STARTED");
//!     printf("%d, %d, %d", first, second, third); // 1, 2, 3
//!     int scooped_from_c = 1;
//!     printf("[C]: ENDED");
//! }
//!
//! ```
//!
//!
//!
//!
//!
//! ## Cons
//!
//! You will probably need to know a lot when it comes to types.
//! What do I mean?
//! Because Merge is so generic, you'll need to manage your types manually
//! Which is expected because of the philosophical difference between languages.
//! (e.g) Python doesn't have types while Rust strictly does.

mod inline;
mod log;
mod parser;
mod prep;
mod task;
mod tokenizer;

use task::Task;

use crate::tokenizer::{Token, Tokenizer};

fn main() {
    //     WHY???
    //     ![
    //       //   rust = {
    //             build = "hello world",
    //         },
    //     ]
    //     ERR! (use unicode segmentation)
    //     ![
    //       ½   rust = {
    //             build = "hello world",
    //         },
    //     ]
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
            +++

            [scooped]
            c! {
                printf("Hello, World!");
                {},
                printf("%s", scooped);
                char* scooped = "C STRING";
            }

        "###
    .to_string();

    let mut tokenizer = Tokenizer::new(rstr);
    let mut invalid_buf = Vec::new();

    let tokens = tokenizer.tokenize();
    tokens.iter().for_each(|i| {
        if let Token::Invalid(ch) = i {
            invalid_buf.push(ch);
        }
    });

    let pp = prep::tasks::PreprocessorTask::new();
    let mut task_manager = task::TaskManager::new();

    task_manager.push(Box::new(pp));

    task_manager.run_all();

    println!("{tokens:#?}");
    println!("");
    println!("Invalid Tokens: {:#?}", invalid_buf);

    // let mut parser = Parser::from(tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut lexer = Tokenizer::new(rstr);

        let tokens = lexer.tokenize();
        tokens.iter().for_each(|i| {
            if let Token::Invalid(ch) = i {
                panic!("Invalid token: {}", ch);
            }
        });
    }
}
