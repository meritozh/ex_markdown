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
        let mut pending = vec![root.clone()];
        while !pending.is_empty() {
            let mut inlines = vec![];

            pending.iter().for_each(|node_id| {
                if let Ok(children) = self.tree.children_ids(node_id) {
                    children.for_each(|node_id| {
                        if self.tree.need_pass_down(node_id) {
                            match self.tree.get(&node_id).unwrap().data() {
                                Token::Block(b) => match b {
                                    Block::BlockQuote(blockquote) => {
                                        let tokens = parse_inline(blockquote.content);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                    }
                                    Block::List(list) => {
                                        let tokens = parse_inline(list.content);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                    }
                                    Block::Paragraph(paragraph) => {
                                        let tokens = parse_inline(paragraph.content);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                    }
                                    _ => unreachable!(),
                                },
                                Token::Inline(i) => match i {
                                    // TODO: Link cannot embed another Link
                                    Inline::Link(link) => {
                                        let tokens = parse_inline(link.label);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                        if let Some(title) = link.title {
                                            let tokens = parse_inline(title);
                                            tokens.into_iter().for_each(|t| {
                                                inlines.push((t, node_id.clone()));
                                            });
                                        }
                                    }
                                    Inline::Mark(mark) => {
                                        let tokens = parse_inline(mark.content);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                    }
                                    Inline::Diff(diff) => {
                                        let tokens = parse_inline(diff.content);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                    }
                                    // TODO: Ruby cannot embed another Ruby
                                    Inline::Ruby(ruby) => {
                                        let tokens = parse_inline(ruby.content);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                    }
                                    // TODO: Image cannot embed another Image
                                    Inline::Image(image) => {
                                        let tokens = parse_inline(image.label);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                        if let Some(title) = image.title {
                                            let tokens = parse_inline(title);
                                            tokens.into_iter().for_each(|t| {
                                                inlines.push((t, node_id.clone()));
                                            });
                                        }
                                    }
                                    Inline::Emphasis(emphasis) => {
                                        let tokens = parse_inline(emphasis.content);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                    }
                                    Inline::Strikethrough(strikethrough) => {
                                        let tokens = parse_inline(strikethrough.content);
                                        tokens.into_iter().for_each(|t| {
                                            inlines.push((t, node_id.clone()));
                                        });
                                    }
                                    _ => {}
                                },
                                _ => unreachable!(),
                            }
                        }
                    })
                }
            });

            pending = inlines
                .iter()
                .filter(|(_, node_id)| self.tree.need_pass_down(node_id))
                .map(|(_, node_id)| node_id.clone())
                .collect();

            inlines.into_iter().for_each(|(t, ref node_id)| {
                self.tree.push_inline(t, node_id);
            });
        }
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

    fn need_pass_down(&self, node: &NodeId) -> bool;
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

    fn need_pass_down(&self, node_id: &NodeId) -> bool {
        if let Ok(t) = self.get(node_id) {
            return match t.data() {
                Token::Document => true,
                Token::Block(b) => match b {
                    Block::List(_) => true,
                    Block::Paragraph(_) => true,
                    Block::BlockQuote(_) => true,
                    Block::TOC => false,
                    Block::Import(_) => false,
                    Block::BlankLine => false,
                    Block::Heading(_) => false,
                    Block::Command(_) => false,
                    Block::Footnote(_) => false,
                    Block::Container(_) => false,
                    Block::CodeBlock(_) => false,
                    Block::Definition(_) => false,
                    Block::LatexBlock(_) => false,
                    Block::ThematicBreak => false,
                    Block::FrontMatter(_) => false,
                },
                Token::Inline(i) => match i {
                    Inline::Link(_) => true,
                    Inline::Mark(_) => true,
                    Inline::Diff(_) => true,
                    Inline::Ruby(_) => true,
                    Inline::Image(_) => true,
                    Inline::Emphasis(_) => true,
                    Inline::Strikethrough(_) => true,
                    Inline::Span(_) => false,
                    Inline::Text(_) => false,
                    Inline::Latex(_) => false,
                    Inline::Reference(_) => false,
                    Inline::Subscript(_) => false,
                    Inline::Superscript(_) => false,
                },
            };
        }
        return false;
    }
}
