use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, not_line_ending},
    combinator::map,
    error::context,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::token::{Block, CodeBlock};

fn attributes(input: &str) -> IResult<&str, Vec<&str>> {
    not_line_ending(input).map(|(remain, sub)| (remain, sub.split_whitespace().collect()))
}

fn code_block(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    context(
        "code block",
        preceded(
            tag("```"),
            separated_pair(
                attributes,
                line_ending,
                // TODO: use peek see if there's a line_end before delimiter
                terminated(take_until("```"), tuple((tag("```"), line_ending))),
            ),
        ),
    )(input)
}

#[test]
fn code_block_test() {
    assert_eq!(
        code_block("```\nasd\nasd\n```\n"),
        Ok(("", (vec![], "asd\nasd\n")))
    );
    assert_eq!(
        code_block("```c++\nasd\nasd\n```\n"),
        Ok(("", (vec!["c++"], "asd\nasd\n")))
    );
}

pub fn parse_code_block(input: &str) -> IResult<&str, Block> {
    map(code_block, |(attributes, content)| {
        Block::CodeBlock(CodeBlock {
            attributes,
            content,
        })
    })(input)
}
