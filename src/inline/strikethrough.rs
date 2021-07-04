use nom::{
    character::complete::{anychar, char},
    combinator::{map, verify},
    error::context,
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult,
};

#[cfg(test)]
use nom::{
    error::{Error, ErrorKind},
    Err,
};

use crate::token::{Inline, Strikethrough};

fn strikethrough(input: &str) -> IResult<&str, &str> {
    context(
        "strike through",
        map(
            verify(
                tuple((
                    many1_count(char('~')),
                    many_till(anychar, many1_count(char('~'))),
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
fn strikethrough_test() {
    assert_eq!(strikethrough("~~test~~"), Ok(("", "test")));
    assert_eq!(
        strikethrough("~test~"),
        Err(Err::Error(Error::new("~test~", ErrorKind::Verify)))
    );
}

pub fn parse_strikethrough(input: &str) -> IResult<&str, Inline> {
    map(strikethrough, |content| {
        Inline::Strikethrough(Strikethrough {
            content,
            ..Default::default()
        })
    })(input)
}
