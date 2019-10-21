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

use crate::token::{Bold, Inline};

fn bold(input: &str) -> IResult<&str, &str> {
    context(
        "bold",
        delimited(
            // TODO: We need only consume last `_` if it is an sequence.
            char('*'),
            // TODO: Change implementation way.
            verify(take_till1(|ch| ch == '*'), |s: &str| {
                space1::<_, (&str, ErrorKind)>(s).is_err()
            }),
            char('*'),
        ),
    )(input)
}

#[test]
fn bold_test() {
    assert_eq!(bold("*test*"), Ok(("", "test")));
    assert_eq!(
        bold("**test*"),
        Err(Err::Error(("*test*", ErrorKind::TakeTill1)))
    );
    // FIXME: It should be acceptable in new implementation way.
    assert_eq!(bold("*  *"), Err(Err::Error(("  *", ErrorKind::Verify))));
}

pub fn parse_bold(input: &str) -> IResult<&str, Inline> {
    map(bold, |s| Inline::Bold(Bold { content: s }))(input)
}
