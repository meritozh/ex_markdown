use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, line_ending, not_line_ending, space1},
    combinator::{map, map_parser, peek, rest},
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::token::{Block, Footnote};

fn footnote(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "footnote",
        tuple((
            delimited(
                char('['),
                map_parser(take_until("]:"), preceded(peek(char('^')), rest)),
                tuple((tag("]:"), space1)),
            ),
            terminated(not_line_ending, line_ending),
        )),
    )(input)
}

#[test]
fn footnote_test() {
    assert_eq!(footnote("[^test]: hahaha\n"), Ok(("", ("^test", "hahaha"))))
}

pub fn parse_footnote(input: &str) -> IResult<&str, Block> {
    map(terminated(footnote, line_ending), |(label, content)| {
        Block::Footnote(Footnote { label, content })
    })(input)
}
