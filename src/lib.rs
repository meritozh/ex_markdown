mod block;
mod inline;
mod parser;
mod token;

pub(crate) mod utils;

use std::fmt::Result;

use parser::Parser;

pub fn parse_markdown(input: &str) -> Result {
    let mut parser = Parser::default();
    parser.ext(input);

    let json = serde_json::to_string_pretty(&parser.tree).unwrap();
    println!("{}", json);

    Ok(())
}
