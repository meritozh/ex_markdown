use nom::{
    bytes::complete::{tag, take_until},
    character::complete::line_ending,
    combinator::map,
    error::context,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::token::{Block, LatexBlock};

fn latex_block<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    context(
        "latex block",
        preceded(
            tuple((tag("$$"), line_ending)),
            terminated(take_until("$$"), tuple((tag("$$"), line_ending))),
        ),
    )(input)
}

#[test]
fn latex_block_test() {
    assert_eq!(
        latex_block("$$\nlatex here\n$$\n"),
        Ok(("", "latex here\n"))
    );
}

pub(super) fn parse_latex_block(input: &str) -> IResult<&str, Block> {
    map(latex_block, |content| {
        Block::LatexBlock(LatexBlock { content })
    })(input)
}
