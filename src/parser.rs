use id_tree::{InsertBehavior::*, Node, NodeId, Tree};

use crate::{
    block::{parse_block, parse_front_matter},
    token::{Block, Token},
};

#[derive(Debug, PartialEq, Default)]
pub struct Parser<'a> {
    pub text: &'a str,
    pub remaining: &'a str,

    pub tree: Tree<Token<'a>>,
    pub definitions: Vec<NodeId>,
    pub headings: Vec<NodeId>,
}

impl<'a> Parser<'a> {
    pub fn ext(&mut self, input: &'a str) {
        // store origin input as text
        self.text = input;
        self.remaining = input;

        // construct Document node as root
        let root = self
            .tree
            .insert(Node::new(Token::Document), AsRoot)
            .unwrap();

        self.parse_first_pass(&root);
    }

    fn parse_first_pass(&mut self, root: &NodeId) {
        // must parse frontmatter first.
        parse_front_matter(self, root);

        while let Ok((i, t)) = parse_block(self.remaining) {
            match t {
                Block::Definition(_) => self.push_definition(t, root),
                Block::Heading(_) => self.push_heading(t, root),
                _ => {
                    let _ = self.push_token(t, root);
                }
            };
            self.remaining = i;
        }

        assert!(self.remaining.is_empty());
    }

    fn push_definition(&mut self, t: Block<'a>, root: &NodeId) {
        let node_id = self.push_token(t, root);
        self.definitions.push(node_id);
    }

    fn push_heading(&mut self, t: Block<'a>, root: &NodeId) {
        let node_id = self.push_token(t, root);
        self.headings.push(node_id);
    }

    fn push_token(&mut self, t: Block<'a>, parent: &NodeId) -> NodeId {
        let node_id = self
            .tree
            .insert(Node::new(Token::Block(t)), UnderNode(&parent))
            .unwrap();
        node_id
    }
}
