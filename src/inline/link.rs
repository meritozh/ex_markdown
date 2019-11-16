use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, value},
    error::context,
    multi::many_till,
    sequence::tuple,
    IResult,
};

use crate::{
    inline::{parse_inline, text::text},
    token::{Inline, Link},
};

fn link(input: &str) -> IResult<&str, (&str, &str, &str)> {
    context(
        "link",
        map(
            tuple((
                map(many_till(anychar, tag("[")), |(leading, _)| leading.len()),
                alt((
                    value(0, tag("](")),
                    map(many_till(anychar, tag("](")), |(title, _)| title.len()),
                )),
                map(many_till(anychar, tag(")")), |(uri, _)| uri.len()),
            )),
            |(count1, count2, count3)| {
                (
                    &input[..count1],
                    &input[count1 + 1..count1 + 1 + count2],
                    &input[count1 + 1 + count2 + 2..count1 + 1 + count2 + 2 + count3],
                )
            },
        ),
    )(input)
}

#[test]
fn link_test() {
    assert_eq!(
        link("[](http://example.com)"),
        Ok(("", ("", "", "http://example.com")))
    );
    assert_eq!(
        link("[test](http://example.com)"),
        Ok(("", ("", "test", "http://example.com")))
    );
    assert_eq!(
        link("123[test](http://example.com)"),
        Ok(("", ("123", "test", "http://example.com")))
    );
}

pub fn parse_link(input: &str) -> IResult<&str, Vec<Inline>> {
    map(link, |(leading, title, uri)| {
        vec![
            parse_inline(leading),
            vec![Inline::Link(Link {
                title: text(title),
                uri: text(uri),
            })],
        ]
        .into_iter()
        .flatten()
        .collect()
    })(input)
}
