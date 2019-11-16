use nom::{
    bytes::complete::tag, character::complete::anychar, combinator::map, error::context,
    multi::many_till, sequence::tuple, IResult,
};

use crate::{
    inline::{parse_inline, text::text},
    token::{Inline, Ruby},
};

fn ruby(input: &str) -> IResult<&str, (&str, &str, &str)> {
    context(
        "ruby",
        map(
            tuple((
                map(many_till(anychar, tag("{")), |(leading, _)| leading.len()),
                map(many_till(anychar, tag("|")), |(title, _)| title.len()),
                map(many_till(anychar, tag("}")), |(uri, _)| uri.len()),
            )),
            |(count1, count2, count3)| {
                (
                    &input[..count1],
                    &input[count1 + 1..count1 + 1 + count2],
                    &input[count1 + 1 + count2 + 1..count1 + 1 + count2 + 1 + count3],
                )
            },
        ),
    )(input)
}

#[test]
fn ruby_test() {
    assert_eq!(
        ruby("{test|annotation}"),
        Ok(("", ("", "test", "annotation")))
    );
}

pub fn parse_ruby(input: &str) -> IResult<&str, Vec<Inline>> {
    map(ruby, |(leading, ruby, annotation)| {
        vec![
            parse_inline(leading),
            vec![Inline::Ruby(Ruby {
                children: parse_inline(ruby),
                annotation: text(annotation),
            })],
        ]
        .into_iter()
        .flatten()
        .collect()
    })(input)
}
