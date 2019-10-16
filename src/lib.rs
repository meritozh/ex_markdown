mod block;
mod inline;
mod parser;
mod token;

use parser::Parser;

pub fn markdown(input: &str) -> Parser {
    let mut parser: Parser = Default::default();
    parser.run(input);
    parser
}
