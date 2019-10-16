use crate::{block::parse_block, token::Token};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Parser<'a> {
    pub nodes: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn run(&mut self, input: &'a str) {
        parse_block(self, input)
    }

    pub fn push(&mut self, node: Token<'a>) {
        self.nodes.push(node);
    }
}
