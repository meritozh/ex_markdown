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

use crate::token::{Inline, Subscript};

fn subscript(input: &str) -> IResult<&str, &str> {
    context(
        "subscript",
        map(
            verify(
                tuple((
                    many1_count(char('~')),
                    many_till(anychar, many1_count(char('~'))),
                )),
                |(left, (_, right))| *left == 1 && *right == 1,
            ),
            |(left, (content, _))| &input[left..left + content.len()],
        ),
    )(input)
}

#[test]
fn subscript_test() {
    assert_eq!(subscript("~test~"), Ok(("", "test")));
    assert_eq!(
        subscript("~~test~~"),
        Err(Err::Error(Error::new("~~test~~", ErrorKind::Verify)))
    );
}

pub fn parse_subscript(input: &str) -> IResult<&str, Inline> {
    map(subscript, |content| {
        Inline::Subscript(Subscript { content })
    })(input)
}
