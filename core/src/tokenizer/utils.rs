// //! Utility for tokenizing Merge code.
//
// use super::Tokenizer;
//
// /// # Example
// /// ```rust
// /// # use merge::tokenizer::Tokenizer;
// /// # use merge::tokenizer::utils::check_keyword;
// /// #
// /// # fn main() {
// ///    let mut tokenizer = Tokenizer::new("python!(print(\"Hello World!\")".to_owned()));
// ///    assert_eq!(check_keyword(tokenizer, "python!"), true);
// ///    assert_eq!(check_keyword(tokenizer, "rust"), false);
// /// # }
// #[inline]
// fn check_keyword(tokenizer: &mut Tokenizer, keyword: &str) -> bool {
//     let mut index = 0isize;
//     let mut lit_chars = keyword.chars();
//     for c in lit_chars {
//         if c != tokenizer.offset(index) {
//             return false;
//         }
//         index += 1;
//     }
//
//     // If keyword is valid, consume it
//     for _ in 0..keyword.len() {
//         tokenizer.advance();
//     }
//
//     true
// }
