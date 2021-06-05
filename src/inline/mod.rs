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

use nom::{branch::alt, IResult};

use super::token::Inline;

use self::{
    diff::parse_diff, image::parse_image, latex::parse_latex, link::parse_link, mark::parse_mark,
    reference::parse_reference, ruby::parse_ruby, span::parse_span,
    strikethrough::parse_strikethrough, subscript::parse_subscript, superscript::parse_superscript,
    text::parse_text,
};

pub(crate) fn parse_inline(input: &str) -> IResult<&str, Inline> {
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
