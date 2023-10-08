#![allow(warnings, unused)]

mod log;
mod parser;
mod prep;
mod tokenizer;
mod inline;
mod task;

use regex::Regex;

use crate::parser::Parser;
use crate::prep::PreProcessor;
use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;

// The input text containing the Python code
static CODE: &str = r#"
    rust!(
        use std::io::write;
        fn main() {
            println!("Aus Rust... einheimisch");
        }
    )
    
    python!(
        print("Hallo aus Python")
        # import cv2 as cv
        #                            
        # vid = cv.VideoCapture(0) # 0 is the camera index.
        #                          # For Example:
        #                          # If you have a 2nd camera
        #                          # You can use 1 as index
    )
    
    cpp!( // This part of the code is not parsed intentionally
        #include<iostream>
        
        using namespace std;
        
        int main() {
            cout << "Hallo aus C++" << endl;
        }
    )
"#;

// Disabling regexes greediness had some funny impact on the results.
// For Example:
// python!(
//      print("hello world")
// )
//
// We'd expect this code to match "as is" but that's not the case
// with greediness disable regex engine seems to match:
//
// python!( <--------------------+
//      print("hello world") <---+ Which leaves the parenthesis below urphan
// ) <-- unmatched
//
// So I must write my own parser.
// Which is hard :(

fn main() {
    //     // Sample input tokens representing "2 * (3 + 4)"
    //     let input_tokens = vec![
    //         token::number(2.0),
    //         token::multiply,
    //         token::leftparenthesis,
    //         token::number(3.0),
    //         token::plus,
    //         token::number(4.0),
    //         token::rightparenthesis,
    //         token::endoffile,
    //     ];

    // 2 * (3 + 4) / <- Expected patronum!

    let rstr = r"
    123 * ( 10 + 10 )
    "
    .to_string();

    let mut lexer = Tokenizer::new(rstr);
    let tokens = lexer.tokenize();
    tokens.iter().for_each(|i| {
        if let Token::Invalid(ch) = i {
            panic!("Invalid Token found: {:#?}", ch);
        }
    });

    println!("{tokens:#?}");

    let mut parser = Parser::from(tokens);

    // Parse the code
    let result = parser.parse_expression();
    println!("Result: {result}");
}

// Desired syntax:
//
// python!(
//     print("Hello, World!")
// )
//
// cpp!(
//     #include<iostream>
//
//     using namespace std;
//
//     int main() {
//         cout << "Hello, World!" << endl;
//     }
// )
//
// a = javascript!(
//     console.log("Hello, World!")
// )
//
// [a] rust!( // Like inlining assembly
//      println!("{a}");
//      println!("Hello, World!");
// )
//

//
// let variable = "some_variable"; // <-- Transpiled to rust
//
// #[python] // Native style
// use {
//     matplotlib.pyplot as plt,
//     pandas as pd,
// }
//
// python!(
//     import cv2 as cv
//     import pytesseract.tesseract as tsr
// )
//
// let some = python!(
//     def foo():
//        return 10
//     foo() # TODO: think more about the way I'll handle results.
// )
//
// # This could transpile to this:
// #
// # ```python
// # def bar():
// #    def foo():
// #        return 10
// #    return foo()
// # ```
// # OR
// #
// # def foo():
// #    return 10
// # a = foo()  # Which is way more cleaner but probably impossible to infer?
//
// // After introducing a variable named `some`
// // We need to be albe to send it though a bridge
// // the [<CONTENT>,*] indicates a bridge for the following inliner (e.g python!)
//
// ```merge
// let variable = rust!(
//     use regex::Regex;
//
//     fn main() {
//         let re = Regex::new(r"Hello\s\w");
//         re.match("Hello world");
//     }
// ) // end rust!
//
// let python = [variable] python!(
//     print("From Python: ", variable);
// )
// ```
//
// // The problem is that this syntax is prone
// // to misalignment. (which is a big problem)
//
// [variable] rust!(
//     // Sadly not aligned...
// )
//
// [variable]
// rust!(
//     println!("{variable}");
// )
//
// [variable]
// python!(
//     print("From Python: ", variable)
// )
