use nom::{
    bytes::complete::tag,
    character::complete::{anychar, line_ending, not_line_ending},
    combinator::map,
    error::context,
    multi::many_till,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::token::{Block, Footnote};

fn footnote(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "footnote",
        preceded(
            tag("[^"),
            tuple((
                map(many_till(anychar, tag("]: ")), |(matched, _)| {
                    &input[2..2 + matched.len()]
                }),
                not_line_ending,
            )),
        ),
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
