mod block;
mod inline;
mod parser;
mod token;

pub(crate) mod utils;

use parser::Parser;

pub fn markdown(input: &str) -> Parser {
    let mut parser: Parser = Default::default();

    parser
}
