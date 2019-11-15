use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, verify},
    error::context,
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult,
};

#[cfg(test)]
use nom::{error::ErrorKind, Err};

use crate::{
    inline::{parse_inline, text::text},
    token::{Inline, Span},
};

fn span(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "span",
        map(
            verify(
                tuple((
                    map(
                        many_till(anychar, many1_count(tag("`"))),
                        |(leading, count)| (leading.len(), count),
                    ),
                    many_till(anychar, many1_count(tag("`"))),
                )),
                |((_, count1), (_, count2))| count1 == count2,
            ),
            |((count1, count2), (content, _))| {
                (
                    &input[..count1],
                    &input[count1 + count2..count1 + count2 + content.len()],
                )
            },
        ),
    )(input)
}

#[test]
fn span_test() {
    assert_eq!(span("`test`"), Ok(("", ("", "test"))));
    assert_eq!(span("```test```"), Ok(("", ("", "test"))));
    assert_eq!(span("123```test```"), Ok(("", ("123", "test"))));
    assert_eq!(
        span("``test`"),
        Err(Err::Error(("``test`", ErrorKind::Verify)))
    );
    assert_eq!(
        span("`test``"),
        Err(Err::Error(("`test``", ErrorKind::Verify)))
    );
    assert_eq!(span("`test"), Err(Err::Error(("", ErrorKind::Eof))));
    assert_eq!(span("``"), Err(Err::Error(("", ErrorKind::Eof))));
}

pub fn parse_span(input: &str) -> IResult<&str, Vec<Inline>> {
    map(span, |(leading, content)| {
        vec![
            parse_inline(leading),
            vec![Inline::Span(Span {
                child: text(content),
            })],
        ]
        .into_iter()
        .flatten()
        .collect()
    })(input)
}
