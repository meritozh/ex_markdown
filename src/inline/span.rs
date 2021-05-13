use nom::{
    character::complete::{anychar, char},
    combinator::{map, verify},
    error::context,
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult,
};

use crate::token::{Inline, Span};

fn span(input: &str) -> IResult<&str, &str> {
    context(
        "span",
        map(
            verify(
                tuple((
                    many1_count(char('`')),
                    // TODO: anychar need convert line_ending to space.
                    many_till(anychar, many1_count(char('`'))),
                )),
                |(left, (_, right))| left == right,
            ),
            |(left, (content, _))| &input[left..left + content.len()],
        ),
    )(input)
}

#[test]
fn span_test() {
    assert_eq!(span("`test`"), Ok(("", "test")));
    assert_eq!(span("```test```"), Ok(("", "test")));
}

pub fn parse_span(input: &str) -> IResult<&str, Inline> {
    map(span, |content| Inline::Span(Span { content }))(input)
}
