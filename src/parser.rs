use id_tree::{InsertBehavior::*, Node, NodeId, Tree};

use crate::{
    block::{parse_block, parse_front_matter},
    inline::parse_inline,
    token::{Block, Inline, Token},
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
        let root = self.tree.push_document();

        self.parse_first_pass(&root);
        self.parse_second_pass(&root);
    }

    fn parse_first_pass(&mut self, root: &NodeId) {
        // must parse frontmatter first.
        if let Ok((i, t)) = parse_front_matter(self.text) {
            self.tree.push_block(t, root);
            self.remaining = i;
        }

        while let Ok((i, t)) = parse_block(self.remaining) {
            match t {
                Block::Definition(_) => {
                    let node_id = self.tree.push_block(t, root);
                    self.record_definition(node_id);
                }
                Block::Heading(_) => {
                    let node_id = self.tree.push_block(t, root);
                    self.record_heading(node_id);
                }
                _ => {
                    self.tree.push_block(t, root);
                }
            };
            self.remaining = i;
        }

        assert!(self.remaining.is_empty());
    }

    fn parse_second_pass(&mut self, root: &NodeId) {
        let mut inlines = vec![];

        let blocks = self.tree.children_ids(root).unwrap();

        blocks.for_each(|node_id| {
            let b = self.tree.get(&node_id).unwrap().data();

            if let Token::Block(b) = b {
                println!("{:?}", b);
                match b {
                    // Block::BlockQuote(_) => todo!(),
                    // Block::List(_) => todo!(),
                    // Block::Heading(_) => todo!(),
                    // Block::Import(_) => todo!(),
                    // Block::Command(_) => todo!(),
                    // Block::CodeBlock(_) => todo!(),
                    // Block::LatexBlock(_) => todo!(),
                    // Block::Definition(_) => todo!(),
                    // Block::Footnote(_) => todo!(),
                    // Block::Container(_) => todo!(),
                    // Block::BlankLine => todo!(),
                    // Block::ThematicBreak => todo!(),
                    // Block::TOC => todo!(),
                    Block::Paragraph(paragraph) => {
                        let mut next = paragraph.content;
                        while let Ok((i, t)) = parse_inline(next) {
                            inlines.push((t, node_id.clone()));
                            next = i;
                        }
                    }
                    _ => {}
                }
            }
        });

        inlines.into_iter().for_each(|(t, ref node_id)| {
            self.tree.push_inline(t, node_id);
        });
    }

    fn record_definition(&mut self, node_id: NodeId) {
        self.definitions.push(node_id);
    }

    fn record_heading(&mut self, node_id: NodeId) {
        self.headings.push(node_id);
    }
}

trait PushToken<'a> {
    fn push_document(&mut self) -> NodeId;

    fn push_block(&mut self, t: Block<'a>, parent: &NodeId) -> NodeId;

    fn push_inline(&mut self, t: Inline<'a>, parent: &NodeId) -> NodeId;
}

impl<'a> PushToken<'a> for Tree<Token<'a>> {
    fn push_document(&mut self) -> NodeId {
        self.insert(Node::new(Token::Document), AsRoot).unwrap()
    }

    fn push_block(&mut self, t: Block<'a>, parent: &NodeId) -> NodeId {
        self.insert(Node::new(Token::Block(t)), UnderNode(parent))
            .unwrap()
    }

    fn push_inline(&mut self, t: Inline<'a>, parent: &NodeId) -> NodeId {
        self.insert(Node::new(Token::Inline(t)), UnderNode(parent))
            .unwrap()
    }
}
