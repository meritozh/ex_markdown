use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::map,
    error::context,
    sequence::terminated,
    IResult,
};

use crate::token::{Block, Paragraph};

/// Recognize any characters until EOL or no more data.
fn paragraph(input: &str) -> IResult<&str, &str> {
    context("paragraph", terminated(not_line_ending, line_ending))(input)
}

#[test]
fn paragraph_test() {
    assert_eq!(paragraph("test\n"), Ok(("", "test")));
}

pub(super) fn parse_paragraph(input: &str) -> IResult<&str, Block> {
    map(paragraph, |content| {
        Block::Paragraph(Paragraph {
            content,
            ..Default::default()
        })
    })(input)
}
