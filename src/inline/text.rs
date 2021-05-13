use nom::{
    character::complete::anychar,
    combinator::{eof, map},
    error::context,
    multi::many0,
    sequence::terminated,
    IResult,
};

use crate::token::{Inline, Text};

pub fn text(input: &str) -> IResult<&str, &str> {
    context(
        "text",
        // TODO: anychar need convert line_ending to space.
        terminated(map(many0(anychar), |v| &input[..v.len()]), eof),
    )(input)
}

pub fn parse_text(input: &str) -> IResult<&str, Inline> {
    map(text, |content| Inline::Text(Text { content }))(input)
}
