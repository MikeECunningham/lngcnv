// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::cognitive_complexity, clippy::missing_panics_doc, clippy::too_many_lines)]

// IMPORTS


pub mod lang;
use crate::lang::*;

enum Language {
  English
}

pub fn english_ipa(input: String) -> String {
  let text: &str = input.trim();
  modeng::engaucanberra(&text)
}