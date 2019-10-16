use nom::{
    bytes::complete::take_till1,
    character::complete::{char, space1},
    combinator::{map, verify},
    error::{context, ErrorKind},
    sequence::delimited,
    IResult,
};

#[cfg(test)]
use nom::Err;

use crate::token::{Inline, Italic};

fn italic(input: &str) -> IResult<&str, &str> {
    context(
        "italic",
        delimited(
            // TODO: We need only consume last `_` if it is an sequence.
            char('_'),
            verify(take_till1(|ch| ch == '_'), |s: &str| {
                space1::<_, (&str, ErrorKind)>(s).is_err()
            }),
            char('_'),
        ),
    )(input)
}

#[test]
fn italic_test() {
    assert_eq!(italic("_test_"), Ok(("", "test")));
    assert_eq!(
        italic("__test_"),
        Err(Err::Error(("_test_", ErrorKind::TakeTill1)))
    );
    assert_eq!(italic("_  _"), Err(Err::Error(("  _", ErrorKind::Verify))));
}

pub fn parse_italic(input: &str) -> IResult<&str, Inline> {
    map(italic, |s| Inline::Italic(Italic { content: s }))(input)
}
