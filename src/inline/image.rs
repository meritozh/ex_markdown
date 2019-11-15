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
    token::{Image, Inline},
};

fn image(input: &str) -> IResult<&str, (&str, &str, &str)> {
    context(
        "image",
        map(
            tuple((
                map(many_till(anychar, tag("![")), |(leading, _)| leading.len()),
                alt((
                    value(0, tag("](")),
                    map(many_till(anychar, tag("](")), |(title, _)| title.len()),
                )),
                map(many_till(anychar, tag(")")), |(uri, _)| uri.len()),
            )),
            |(count1, count2, count3)| {
                (
                    &input[..count1],
                    &input[count1 + 2..count1 + 2 + count2],
                    &input[count1 + 2 + count2 + 2..count1 + 2 + count2 + 2 + count3],
                )
            },
        ),
    )(input)
}

#[test]
fn image_test() {
    assert_eq!(
        image("![test](http://example.com)"),
        Ok(("", ("", "test", "http://example.com")))
    );
    assert_eq!(
        image("![](http://example.com)"),
        Ok(("", ("", "", "http://example.com")))
    );
}

pub fn parse_image(input: &str) -> IResult<&str, Vec<Inline>> {
    map(image, |(leading, title, uri)| {
        vec![
            parse_inline(leading),
            vec![Inline::Image(Image {
                title: text(title),
                uri: text(uri),
            })],
        ]
        .into_iter()
        .flatten()
        .collect()
    })(input)
}
