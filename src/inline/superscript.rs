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

use crate::token::{Inline, Superscript};

fn superscript(input: &str) -> IResult<&str, &str> {
    context(
        "superscript",
        map(
            verify(
                tuple((
                    many1_count(char('^')),
                    many_till(anychar, many1_count(char('^'))),
                )),
                |(left, (_, right))| *left == 1 && *right == 1,
            ),
            |(left, (content, _))| &input[left..left + content.len()],
        ),
    )(input)
}

#[test]
fn superscript_test() {
    assert_eq!(superscript("^test^"), Ok(("", "test")));
    assert_eq!(
        superscript("^^test^^"),
        Err(Err::Error(Error::new("^^test^^", ErrorKind::Verify)))
    );
}

pub fn parse_superscript(input: &str) -> IResult<&str, Inline> {
    map(superscript, |content| {
        Inline::Superscript(Superscript { content })
    })(input)
}
