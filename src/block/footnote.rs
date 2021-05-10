use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, not_line_ending, space1},
    combinator::map,
    error::context,
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::token::{Block, Footnote};

fn footnote(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "footnote",
        tuple((
            delimited(tag("[^"), take_until("]:"), tuple((tag("]:"), space1))),
            terminated(not_line_ending, line_ending),
        )),
    )(input)
}

#[test]
fn footnote_test() {
    assert_eq!(footnote("[^test]: hahaha"), Ok(("", ("test", "hahaha"))))
}

pub fn parse_footnote(input: &str) -> IResult<&str, Block> {
    map(terminated(footnote, line_ending), |(tag, content)| {
        Block::Footnote(Footnote { tag, content })
    })(input)
}
