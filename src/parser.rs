extern crate unicode_segmentation;

// https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html
use unicode_segmentation::UnicodeSegmentation;

use super::transformation::error::TransformError::{Fail,LR};
use super::transformation::tokenizer::tokenizer;
use super::transformation::transform::transform;
use super::transformation::scope::Scope;
use super::runtime::{Value};

pub fn parse<'a>(code: &'static str) {
  let graphemes = UnicodeSegmentation::graphemes(code, true)
    .collect::<Vec<&str>>();

  let input = graphemes
    .iter()
    .map(|c| Value::String(c.to_string()))
    .collect();

  let mut scope = Scope::new(input);
  let res = transform(&mut scope, tokenizer());
  
  match res {
    Ok(v) => {
      println!("Pattern matched: {:?}", v);
    },
    Err(e) => {
      match e {
        Fail => {
          println!("Failed to parse successfully.");
        },
        LR => panic!("Invalid Left Recursion.")
      }
    }
  }

  match scope.peek() {
    Some(_v) => println!("Failed to read to end of input."),
    None => println!("Successfully parsed all input.")
  }
}
