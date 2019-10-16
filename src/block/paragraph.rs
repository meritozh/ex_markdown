use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::map,
    sequence::terminated,
    IResult,
};

use crate::token::{Block, Paragraph};

/// Recognize any characters until EOL or no more data.
fn paragraph(input: &str) -> IResult<&str, &str> {
    not_line_ending(input)
}

#[test]
fn paragraph_test() {
    assert_eq!(paragraph("test"), Ok(("", "test")));
}

pub fn parse_paragraph(input: &str) -> IResult<&str, Block> {
    map(terminated(paragraph, line_ending), |content| {
        Block::Paragraph(Paragraph { child: content })
    })(input)
}
