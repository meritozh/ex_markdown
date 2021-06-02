mod diff;
mod emphasis;
mod image;
mod latex;
mod link;
mod mark;
mod reference;
mod ruby;
mod span;
mod strikethrough;
mod subscript;
mod superscript;
mod text;

mod shared;

use id_tree::NodeId;
use nom::{branch::alt, IResult};

use super::{
    token::{Block, Inline, Token},
    Parser,
};

use self::{
    diff::parse_diff, image::parse_image, latex::parse_latex, link::parse_link, mark::parse_mark,
    reference::parse_reference, ruby::parse_ruby, span::parse_span,
    strikethrough::parse_strikethrough, subscript::parse_subscript, superscript::parse_superscript,
    text::parse_text,
};

fn parse_inline(input: &str) -> IResult<&str, Inline> {
    alt((
        parse_diff,
        parse_latex,
        parse_link,
        parse_image,
        parse_mark,
        parse_reference,
        parse_ruby,
        parse_span,
        parse_strikethrough,
        parse_subscript,
        parse_superscript,
        parse_text,
    ))(input)
}

pub(crate) fn parse(root: &NodeId, parser: &mut Parser) {
    let blocks = parser.tree.children(root).unwrap();
    blocks.for_each(|token| {
        let b = token.data();

        if let Token::Block(b) = b {
            match b {
                Block::FrontMatter(_) => todo!(),
                Block::Paragraph(_) => todo!(),
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
            }
        }
    });
}
