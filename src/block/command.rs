use nom::{
    bytes::{complete::take_until, streaming::tag},
    character::{complete::line_ending, streaming::not_line_ending},
    combinator::map,
    error::context,
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

use crate::token::{Block, Command};

fn command(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "command",
        pair(
            delimited(tag("<<< "), not_line_ending, line_ending),
            terminated(take_until("<<<"), tuple((tag("<<<"), line_ending))),
        ),
    )(input)
}

#[test]
fn command_test() {
    assert_eq!(
        command("<<< chart\nasfdgsehr\n<<<\n"),
        Ok(("", ("chart", "asfdgsehr\n")))
    );
}

pub(crate) fn parse_command(input: &str) -> IResult<&str, Block> {
    map(command, |(tag, content)| {
        Block::Command(Command { tag, content })
    })(input)
}
