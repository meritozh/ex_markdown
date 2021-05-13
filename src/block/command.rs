use nom::{
    bytes::complete::take_until,
    character::complete::{char, line_ending},
    combinator::map,
    error::context,
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::token::{Block, Command};

fn command(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "command",
        tuple((
            delimited(char('#'), take_until("{"), tuple((char('{'), line_ending))),
            // TODO: use peek see if there's a line_end before delimiter
            terminated(take_until("}"), tuple((char('}'), line_ending))),
        )),
    )(input)
}

#[test]
fn command_test() {
    assert_eq!(
        command("#chart{chart info here\n}\n"),
        Ok(("", ("chart", "chart info here\n")))
    );
}

pub fn parse_command(input: &str) -> IResult<&str, Block> {
    map(command, |(tag, content)| {
        Block::Command(Command { tag, content })
    })(input)
}
