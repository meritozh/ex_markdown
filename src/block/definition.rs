use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, line_ending, not_line_ending, space0, space1},
    combinator::{map, map_parser, not, peek, rest},
    error::context,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::token::{Block, Definition};

fn str(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(char('\''), take_until("'")),
        preceded(char('"'), take_until("\"")),
    ))(input)
}

fn definition(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    context(
        "definition",
        tuple((
            delimited(
                char('['),
                map_parser(take_until("]:"), preceded(peek(not(char('^'))), rest)),
                tuple((tag("]:"), space1)),
            ),
            map_parser(
                terminated(not_line_ending, line_ending),
                pair(take_until(" "), preceded(space0, str)),
            ),
        )),
    )(input)
}

#[test]
fn reference_test() {
    assert_eq!(
        definition("[ref]: url title"),
        Ok(("", ("ref", ("url", "title"))))
    );
}

pub fn parse_definition(input: &str) -> IResult<&str, Block> {
    map(definition, |(label, (url, title))| {
        Block::Definition(Definition { label, url, title })
    })(input)
}
