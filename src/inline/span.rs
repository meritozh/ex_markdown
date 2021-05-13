use nom::{
    character::complete::{anychar, char},
    combinator::{map, verify},
    error::{context, ParseError},
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult, Parser,
};

use crate::{
    inline::{parse_inline, text::text},
    token::{Inline, Span},
};

fn factory<'a, E: ParseError<&'a str>>(symbol: char) -> impl Parser<&'a str, (usize, usize), E> {
    map(
        verify(
            tuple((
                many1_count(char(symbol)),
                // TODO: anychar need convert line_ending to space.
                many_till(anychar, many1_count(char(symbol))),
            )),
            |(left, (_, right))| left == right,
        ),
        |(left, (content, _))| (left, content.len()),
    )
}

fn span(input: &str) -> IResult<&str, &str> {
    context(
        "span",
        map(factory('`'), |(left, content)| &input[left..left + content]),
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
