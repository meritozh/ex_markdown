use nom::{
    character::complete::{anychar, char, line_ending, not_line_ending},
    combinator::{map, map_parser, not, value, verify},
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
                    terminated(many1_count(char('-')), not(anychar)),
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
        Err(Err::Error(Error::new(" ---", ErrorKind::Many1Count)))
    );
    assert_eq!(
        thematic_break("--\n"),
        Err(Err::Error(Error::new("--\n", ErrorKind::Verify)))
    );
    assert_eq!(
        thematic_break("-----4\n"),
        Err(Err::Error(Error::new("4", ErrorKind::Not)))
    );
}

pub(super) fn parse_thematic_break(input: &str) -> IResult<&str, Block> {
    map(thematic_break, |_| Block::ThematicBreak)(input)
}
