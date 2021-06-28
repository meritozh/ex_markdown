use id_tree::{ChildrenIds, InsertBehavior::*, Node, NodeId, Tree};

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

    fn parse_second_pass(&mut self, parent: &NodeId) {
        let mut inlines = vec![];

        let iter = self.tree.traverse_level_order_ids(parent).unwrap();
        iter.for_each(|node_id| {
            if let Ok(children) = self.tree.children_ids(&node_id) {
                children.for_each(|node_id| {
                    if !self.tree.is_leaf_node(&node_id) {
                        let token = self.tree.get(&node_id).unwrap().data();

                        match token {
                            Token::Block(b) => match b {
                                Block::FrontMatter(_) => todo!(),
                                Block::BlockQuote(_) => todo!(),
                                Block::List(_) => todo!(),
                                Block::Heading(_) => todo!(),
                                Block::Import(_) => todo!(),
                                Block::Command(_) => todo!(),
                                Block::CodeBlock(_) => todo!(),
                                Block::LatexBlock(_) => todo!(),
                                Block::Definition(_) => todo!(),
                                Block::Footnote(_) => todo!(),
                                Block::Container(_) => todo!(),
                                Block::BlankLine => todo!(),
                                Block::ThematicBreak => todo!(),
                                Block::TOC => todo!(),
                                Block::Paragraph(paragraph) => {
                                    let tokens = parse_inline(paragraph.content);
                                    tokens.into_iter().for_each(|t| {
                                        inlines.push((t, node_id.clone()));
                                    });
                                }
                            },
                            Token::Inline(i) => match i {
                                Inline::Text(_) => todo!(),
                                Inline::Link(_) => todo!(),
                                Inline::Emphasis(_) => todo!(),
                                Inline::Mark(_) => todo!(),
                                Inline::Strikethrough(_) => todo!(),
                                Inline::Diff(_) => todo!(),
                                Inline::Image(_) => todo!(),
                                Inline::Ruby(_) => todo!(),
                                Inline::Span(_) => todo!(),
                                Inline::Reference(_) => todo!(),
                                Inline::Subscript(_) => todo!(),
                                Inline::Superscript(_) => todo!(),
                                Inline::Latex(_) => todo!(),
                            },
                            _ => unreachable!(),
                        }
                    }
                })
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

    fn is_leaf_node(&self, node: &NodeId) -> bool;
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

    fn is_leaf_node(&self, node_id: &NodeId) -> bool {
        if let Ok(t) = self.get(node_id) {
            return match t.data() {
                Token::Document => false,
                Token::Block(_) => false,
                Token::Inline(i) => match i {
                    Inline::Link(_) => false,
                    Inline::Mark(_) => false,
                    Inline::Diff(_) => false,
                    Inline::Ruby(_) => false,
                    Inline::Image(_) => false,
                    Inline::Emphasis(_) => false,
                    Inline::Strikethrough(_) => false,
                    Inline::Span(_) => true,
                    Inline::Text(_) => true,
                    Inline::Latex(_) => true,
                    Inline::Reference(_) => true,
                    Inline::Subscript(_) => true,
                    Inline::Superscript(_) => true,
                },
            };
        }
        return false;
    }
}
