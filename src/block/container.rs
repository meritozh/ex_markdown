use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, not_line_ending, space0},
    combinator::map,
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::token::{Block, Container};

fn container<'a>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    context(
        "container",
        tuple((
            delimited(tag(":::"), preceded(space0, not_line_ending), line_ending),
            // TODO: use peek see if there's a line_end before delimiter
            terminated(take_until(":::"), tuple((tag(":::"), line_ending))),
        )),
    )(input)
}

#[test]
fn container_test() {
    assert_eq!(
        container("::: warning\ncontent here\n:::\n"),
        Ok(("", ("warning", "content here")))
    );
}

pub fn parse_container(input: &str) -> IResult<&str, Block> {
    map(container, |(title, content)| {
        Block::Container(Container { title, content })
    })(input)
}
