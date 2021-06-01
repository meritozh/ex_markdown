use id_tree::{InsertBehavior::*, Node, NodeId, Tree};

use crate::{
    block::{parse_first_pass, parse_front_matter},
    token::Token,
};

#[derive(Debug, PartialEq)]
pub struct Parser<'a> {
    pub tree: Tree<Token<'a>>,
    pub definitions: Vec<NodeId>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            tree: Tree::new(),
            definitions: vec![],
        }
    }

    pub fn ext(&mut self, input: &'a str) {
        // construct Document node as root
        let root = self
            .tree
            .insert(Node::new(Token::Document(input)), AsRoot)
            .unwrap();

        let mut next = input;

        // front_matter must parse first
        next = parse_front_matter(next, &root, self);

        // block level first pass
        next = parse_first_pass(next, &root, self);

        // block parser should consume all input
        assert!(next.is_empty());
    }
}
