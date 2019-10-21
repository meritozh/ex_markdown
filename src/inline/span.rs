use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map,
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult,
};

#[cfg(test)]
use nom::{error::ErrorKind, Err};

use crate::{
    inline::text::parse_text,
    token::{Inline, Span},
};

fn span(input: &str) -> IResult<&str, &str> {
    map(
        tuple((many1_count(tag("`")), many_till(anychar, tag("`")))),
        |(count1, (content, _))| &input[count1..count1 + content.len()],
    )(input)
}

#[test]
fn span_test() {
    assert_eq!(span("`test`"), Ok(("", "test")));
    assert_eq!(span("``test`"), Ok(("", "test")));
    assert_eq!(span("`test``"), Ok(("`", "test")));
    assert_eq!(span("````test``"), Ok(("`", "test")));
    assert_eq!(span("`test"), Err(Err::Error(("", ErrorKind::Eof))));
}

pub fn parse_span(input: &str) -> IResult<&str, Inline> {
    map(span, |content| {
        Inline::Span(Span {
            child: parse_text(content),
        })
    })(input)
}
