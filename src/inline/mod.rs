pub(crate) mod diff;
pub(crate) mod image;
pub(crate) mod latex;
pub(crate) mod link;
pub(crate) mod mark;
pub(crate) mod reference;
pub(crate) mod ruby;
pub(crate) mod span;
pub(crate) mod strikethrough;
pub(crate) mod subscript;
pub(crate) mod superscript;
pub(crate) mod text;

mod shared;

use nom::branch::alt;

use crate::token::Inline;

use self::{
    diff::parse_diff, image::parse_image, latex::parse_latex, link::parse_link, mark::parse_mark,
    reference::parse_reference, ruby::parse_ruby, span::parse_span,
    strikethrough::parse_strikethrough, subscript::parse_subscript, superscript::parse_superscript,
    text::parse_text,
};

pub fn parse_inline(input: &str) -> Vec<Inline> {
    let mut cur_input = input;
    let mut tokens: Vec<Inline> = Vec::new();

    while !cur_input.is_empty() {
        let (next_input, token) = alt((
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
        ))(cur_input)
        .unwrap();
        tokens.push(token);
        cur_input = next_input;
    }

    tokens
}
