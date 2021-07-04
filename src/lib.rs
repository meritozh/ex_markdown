mod block;
mod inline;
mod parser;
mod token;

pub(crate) mod utils;

use std::fmt::Result;

use parser::Parser;

pub fn parse_markdown(input: &str) -> Result {
    let mut parser = Parser::default();
    let mut s = String::new();
    parser.ext(input);
    println!("{}", s);
    Ok(())
}
