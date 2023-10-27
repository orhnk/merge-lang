// use crate::{task::Task, tokenizer::Token};
//
// use super::PreprocessorOpts;
//
// pub struct PreprocessorTask<'a> {
//     content: &'a mut Vec<Token>,
//     todo: PreprocessorOpts,
// }
//
// impl<'a> PreprocessorTask<'a> {
//     pub fn new(content: &'a mut Vec<Token>) -> Self
//     where
//         Self: Sized,
//     {
//         Self {
//             content,
//             todo: Default::default(),
//         }
//     }
//
//     pub fn content(&mut self, content: Vec<Token>) {
//         *self.content = content;
//     }
// }
//
// impl Task for PreprocessorTask<'_> {
//     fn run(&mut self) {
//         unsafe {
//             &mut *(self.content as *mut Vec<Token>);
//             let mut i = 0;
//             while i < self.content.len() {
//                 let token = &self.content[i];
//
//                 if let Token::Comment(_) = token {
//                     self.content.remove(i);
//                     continue;
//                 }
//
//                 i += 1;
//             }
//
//             self.content = &mut self.content;
//         }
//     }
// }
