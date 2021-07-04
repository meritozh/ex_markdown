use nom::{
    character::complete::{char, line_ending, not_line_ending, space1},
    combinator::map,
    error::context,
    multi::many1_count,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::token::{Block, BlockQuote};

fn blockquote(input: &str) -> IResult<&str, (usize, &str)> {
    context(
        "blockquote",
        terminated(
            separated_pair(many1_count(char('>')), space1, not_line_ending),
            line_ending,
        ),
    )(input)
}

#[test]
fn blockquote_test() {
    assert_eq!(
        blockquote("> asdasdasd\n> 123asd"),
        Ok(("> 123asd", (1, "asdasdasd")))
    );
}

pub(super) fn parse_blockquote(input: &str) -> IResult<&str, Block> {
    map(blockquote, |(level, content)| {
        Block::BlockQuote(BlockQuote {
            level,
            content,
            ..Default::default()
        })
    })(input)
}
