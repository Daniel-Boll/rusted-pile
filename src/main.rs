use miette::Result as MietteResult;
use rusted_pile::{grammar, lexer, parser::SLR::SLR};
use std::fs;

fn main() -> MietteResult<(), Box<dyn std::error::Error>> {
  let lang_contents = fs::read_to_string("assets/lang/test.pile")?;
  let tokens = lexer::generate::compute_tokens(&lang_contents)?;

  let glc_contents = fs::read_to_string("assets/glc/test.glc")?;
  let mut glc = grammar::parser::parse(&glc_contents)?;
  glc.expand();

  // println!("{:#?}", tokens);
  println!("{glc}");

  SLR::new(glc);

  Ok(())
}
