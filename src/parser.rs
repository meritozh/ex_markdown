use id_tree::{InsertBehavior::*, Node, Tree};

use crate::{block, token::Token};

#[derive(Default, Debug, PartialEq)]
pub struct Parser<'a> {
    tree: Tree<Token<'a>>,
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
        if let Ok((i, token)) = block::parse_front_matter(next) {
            let _ = self
                .tree
                .insert(Node::new(Token::Block(token)), UnderNode(&root));
            next = i;
        }

        // block level first pass
        while let Ok((i, token)) = block::parse_first_pass(next) {
            let _ = self
                .tree
                .insert(Node::new(Token::Block(token)), UnderNode(&root));
            next = i;
        }
    }
}
