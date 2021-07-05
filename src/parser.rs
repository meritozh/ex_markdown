use crate::{
    block::{parse_block, parse_front_matter},
    inline::parse_inline,
    token::{Block, Document, Inline},
};

#[derive(Debug, PartialEq, Default)]
pub struct Parser<'a> {
    pub text: &'a str,
    pub remaining: &'a str,

    pub tree: Document<'a>,
}

impl<'a> Parser<'a> {
    pub fn ext(&mut self, input: &'a str) {
        // store origin input as text
        self.text = input;
        self.remaining = input;

        self.parse_first_pass();
        self.parse_second_pass();
    }

    fn parse_first_pass(&mut self) {
        // must parse frontmatter first.
        if let Ok((i, t)) = parse_front_matter(self.text) {
            self.tree.subtree.push(t);
            self.remaining = i;
        }

        while let Ok((i, t)) = parse_block(self.remaining) {
            self.tree.subtree.push(t);
            self.remaining = i;
        }

        assert!(self.remaining.is_empty());
    }

    fn parse_second_pass(&mut self) {
        self.tree.subtree.iter_mut().for_each(|block| match block {
            Block::Paragraph(paragraph) => {
                paragraph.subtree = parse_inline(paragraph.content);
                loop_inline_parser(&mut paragraph.subtree);
            }
            Block::BlockQuote(blockquote) => {
                blockquote.subtree = parse_inline(blockquote.content);
                loop_inline_parser(&mut blockquote.subtree);
            }
            Block::List(list) => {
                list.subtree = parse_inline(list.content);
                loop_inline_parser(&mut list.subtree);
            }
            Block::Heading(heading) => {
                heading.subtree = parse_inline(heading.content);
                loop_inline_parser(&mut heading.subtree);
            }
            Block::Footnote(footnote) => {
                footnote.subtree = parse_inline(footnote.content);
                loop_inline_parser(&mut footnote.subtree);
            }
            _ => {}
        })
    }
}

fn loop_inline_parser(subtree: &mut Vec<Inline>) {
    subtree.iter_mut().for_each(|inline| match inline {
        Inline::Emphasis(emphasis) => {
            emphasis.subtree = parse_inline(emphasis.content);
            loop_inline_parser(&mut emphasis.subtree);
        }
        Inline::Mark(mark) => {
            mark.subtree = parse_inline(mark.content);
            loop_inline_parser(&mut mark.subtree);
        }
        Inline::Strikethrough(strikethrough) => {
            strikethrough.subtree = parse_inline(strikethrough.content);
            loop_inline_parser(&mut strikethrough.subtree);
        }
        Inline::Diff(diff) => {
            diff.subtree = parse_inline(diff.content);
            loop_inline_parser(&mut diff.subtree);
        }
        Inline::Ruby(ruby) => {
            ruby.subtree = parse_inline(ruby.content);
            loop_inline_parser(&mut ruby.subtree);
        }
        _ => {}
    });
}
