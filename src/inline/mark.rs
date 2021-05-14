use nom::{
    character::complete::{anychar, char},
    combinator::{map, verify},
    error::context,
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult,
};

use crate::token::{Inline, Mark};

fn mark(input: &str) -> IResult<&str, &str> {
    context(
        "mark",
        map(
            verify(
                tuple((
                    many1_count(char('=')),
                    // TODO: anychar need convert line_ending to space.
                    many_till(anychar, many1_count(char('='))),
                )),
                |(left, (_, right)): &(usize, (_, usize))| {
                    *left >= 2 && *right >= 2 && left == right
                },
            ),
            |(left, (content, _))| &input[left..left + content.len()],
        ),
    )(input)
}

#[test]
fn mark_test() {
    assert_eq!(mark("==test=="), Ok(("", "test")));
}

pub fn parse_mark(input: &str) -> IResult<&str, Inline> {
    map(mark, |content| Inline::Mark(Mark { content }))(input)
}
