use nom::{
    character::complete::{char, line_ending, not_line_ending},
    combinator::{map, map_parser, value, verify},
    error::context,
    multi::many1_count,
    sequence::terminated,
    IResult,
};

#[cfg(test)]
use nom::{
    error::{Error, ErrorKind},
    Err,
};

use crate::token::Block;

fn thematic_break(input: &str) -> IResult<&str, ()> {
    context(
        "thematic break",
        value(
            (),
            verify(
                map_parser(
                    terminated(not_line_ending, line_ending),
                    many1_count(char('-')),
                ),
                |count| *count >= 3,
            ),
        ),
    )(input)
}

#[test]
fn thematic_break_test() {
    assert_eq!(thematic_break("---\n"), Ok(("", ())));
    assert_eq!(thematic_break("---------\n"), Ok(("", ())));
    assert_eq!(
        thematic_break(" ---\n"),
        Err(Err::Error(Error::new(" ---", ErrorKind::Verify)))
    );
    assert_eq!(
        thematic_break("--"),
        Err(Err::Error(Error::new("--", ErrorKind::Verify)))
    );
}

pub fn parse_thematic_break(input: &str) -> IResult<&str, Block> {
    map(thematic_break, |_| Block::ThematicBreak)(input)
}
