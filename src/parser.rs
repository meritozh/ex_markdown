use id_tree::{InsertBehavior::*, Node, Tree};

use crate::{block, token::Token};

#[derive(Default, Debug, PartialEq)]
pub struct Parser<'a> {
    pub tree: Tree<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn ext(&mut self, input: &'a str) {
        // construct Document node as root
        let root = self
            .tree
            .insert(Node::new(Token::Document(input)), AsRoot)
            .unwrap();

        let mut next = input;

        // front_matter must parse first
        next = block::parse_front_matter(next, &root, &mut self.tree);

        // block level first pass
        next = block::parse_first_pass(next, &root, &mut self.tree);
    }
}
