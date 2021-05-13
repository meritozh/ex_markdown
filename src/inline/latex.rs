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

use crate::token::{Inline, Latex};

fn latex(input: &str) -> IResult<&str, &str> {
    context(
        "latex",
        map(
            verify(
                tuple((
                    many1_count(char('$')),
                    many_till(anychar, many1_count(char('$'))),
                )),
                |(left, (_, right)): &(usize, (_, usize))| left == right,
            ),
            |(left, (content, _))| &input[left..left + content.len()],
        ),
    )(input)
}

#[test]
fn latex_test() {
    assert_eq!(latex("$test$"), Ok(("", "test")));
    assert_eq!(latex("$$$test$$$"), Ok(("", "test")));
    assert_eq!(
        latex("$$test$"),
        Err(Err::Error(Error::new("$$test$", ErrorKind::Verify)))
    );
    assert_eq!(
        latex("$test$$"),
        Err(Err::Error(Error::new("$test$$", ErrorKind::Verify)))
    );
    assert_eq!(
        latex("$test"),
        Err(Err::Error(Error::new("", ErrorKind::Eof)))
    );
    assert_eq!(latex("$$"), Err(Err::Error(Error::new("", ErrorKind::Eof))));
}

pub fn parse_latex(input: &str) -> IResult<&str, Inline> {
    map(latex, |content| Inline::Latex(Latex { content }))(input)
}
