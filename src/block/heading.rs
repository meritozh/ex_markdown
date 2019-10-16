use nom::{
    character::complete::{char, line_ending, not_line_ending},
    combinator::{map, verify},
    error::context,
    multi::many1_count,
    sequence::{separated_pair, terminated},
    IResult,
};

#[cfg(test)]
use nom::{error::ErrorKind, Err};

use crate::token::{Block, Heading};

/// Recognize `(#+)<space><text>\n` style markdown input.
fn heading(input: &str) -> IResult<&str, (usize, &str)> {
    context(
        "heading",
        separated_pair(
            verify(many1_count(char('#')), |&count| count <= 6),
            char(' '),
            verify(not_line_ending, |text: &str| !text.is_empty()),
        ),
    )(input)
}

#[test]
fn heading_test() {
    assert_eq!(heading("### 123"), Ok(("", (3, "123"))));
    assert_eq!(heading("### ### 123"), Ok(("", (3, "### 123"))));
    assert_eq!(heading("###### 123\nabc"), Ok(("\nabc", (6, "123"))));
    assert_eq!(heading("###"), Err(Err::Error(("", ErrorKind::Char))));
    assert_eq!(heading("### "), Err(Err::Error(("", ErrorKind::Verify))));
    assert_eq!(
        heading("######123"),
        Err(Err::Error(("123", ErrorKind::Char)))
    );
    assert_eq!(
        heading("####### 123"),
        Err(Err::Error(("####### 123", ErrorKind::Verify)))
    );
}

pub fn parse_heading(input: &str) -> IResult<&str, Block> {
    map(terminated(heading, line_ending), |(count, content)| {
        Block::Heading(Heading {
            level: count,
            child: content,
        })
    })(input)
}
