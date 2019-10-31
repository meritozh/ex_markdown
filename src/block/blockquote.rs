use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, not_line_ending},
    combinator::{map, map_parser},
    error::context,
    sequence::{preceded, terminated},
    IResult,
};

use crate::token::{Block, BlockQuote};

fn blockquote(input: &str) -> IResult<&str, &str> {
    // TODO: Need handle empty blockquote line.
    context(
        "blockquote",
        map_parser(not_line_ending, |content| {
            preceded(tag(">"), char(' '))(content).map(|(remain, _)| ("", remain))
        }),
    )(input)
}

#[test]
fn blockquote_test() {
    assert_eq!(
        blockquote("> asdasdasd\n> 123asd"),
        Ok(("\n> 123asd", "asdasdasd"))
    );
}

pub fn parse_blockquote(input: &str) -> IResult<&str, Block> {
    map(terminated(blockquote, line_ending), |content| {
        Block::BlockQuote(BlockQuote { child: content })
    })(input)
}
