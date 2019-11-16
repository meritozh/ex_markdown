pub(crate) mod diff;
pub(crate) mod emphasis;
pub(crate) mod image;
pub(crate) mod latex;
pub(crate) mod link;
pub(crate) mod mark;
pub(crate) mod ruby;
pub(crate) mod span;
pub(crate) mod strikethrough;
pub(crate) mod subscript;
pub(crate) mod superscript;
pub(crate) mod text;

use nom::branch::alt;

use crate::{
    inline::{
        diff::parse_diff, emphasis::parse_emphasis, image::parse_image, latex::parse_latex,
        link::parse_link, mark::parse_mark, ruby::parse_ruby, span::parse_span,
        strikethrough::parse_strikethrough, subscript::parse_subscript,
        superscript::parse_superscript, text::parse_text,
    },
    token::Inline,
};

pub fn parse_inline(input: &str) -> Vec<Inline> {
    let mut cur_input = input;
    let mut tokens: Vec<Inline> = Vec::new();
    while !cur_input.is_empty() {
        let (next_input, mut tks) = alt((
            parse_span,
            parse_latex,
            parse_mark,
            parse_diff,
            parse_ruby,
            parse_image,
            parse_link,
            parse_emphasis,
            parse_strikethrough,
            parse_subscript,
            parse_superscript,
            parse_text,
        ))(cur_input)
        .unwrap();
        tokens.append(&mut tks);
        cur_input = next_input;
    }
    tokens
}
