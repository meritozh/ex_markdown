use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::map,
    error::context,
    sequence::terminated,
    IResult,
};

use crate::{
    inline::parse_inline,
    token::{Block, Paragraph},
};

/// Recognize any characters until EOL or no more data.
fn paragraph(input: &str) -> IResult<&str, &str> {
    context("paragraph", not_line_ending)(input)
}

#[test]
fn paragraph_test() {
    assert_eq!(paragraph("test"), Ok(("", "test")));
}

pub fn parse_paragraph(input: &str) -> IResult<&str, Block> {
    map(terminated(paragraph, line_ending), |content| {
        Block::Paragraph(Paragraph {
            children: parse_inline(content),
        })
    })(input)
}
