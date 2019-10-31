use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, peek},
    error::context,
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult,
};

use std::cmp;

use crate::{
    inline::text::text,
    token::{Emphasis, EmphasisStyle, Inline},
};

fn emphasis(input: &str) -> IResult<&str, (&str, &str, Vec<EmphasisStyle>)> {
    context(
        "emphsis",
        map(
            tuple((
                map(
                    many_till(anychar, many1_count(tag("*"))),
                    |(leading, count)| (leading.len(), count),
                ),
                peek(map(
                    many_till(anychar, many1_count(tag("*"))),
                    |(vec, count)| (vec.len(), count),
                )),
            )),
            |((count1, count2), (count3, count4))| {
                let delimit = cmp::min(count2, count4);
                let leading = if count2 <= delimit {
                    &input[..count1]
                } else {
                    &input[..count1 + count2 - delimit]
                };
                let styles = match delimit {
                    1 => vec![EmphasisStyle::Italic],
                    _ if delimit % 2 == 0 => vec![EmphasisStyle::Bold],
                    _ if delimit % 2 == 1 => vec![EmphasisStyle::Italic, EmphasisStyle::Bold],
                    _ => unreachable!(),
                };
                (
                    count3 + delimit,
                    leading,
                    &input[count1 + count2..count1 + count2 + count3],
                    styles,
                )
            },
        ),
    )(input)
    .map(|(remain, (count, leading, content, styles))| {
        (&remain[count..], (leading, content, styles))
    })
}

#[test]
fn emphasis_test() {
    assert_eq!(
        emphasis("**test**"),
        Ok(("", ("", "test", vec![EmphasisStyle::Bold])))
    );
    assert_eq!(
        emphasis("***test**"),
        Ok(("", ("*", "test", vec![EmphasisStyle::Bold])))
    );
    assert_eq!(
        emphasis("**test***"),
        Ok(("*", ("", "test", vec![EmphasisStyle::Bold])))
    );
    assert_eq!(
        emphasis("***test***"),
        Ok((
            "",
            ("", "test", vec![EmphasisStyle::Italic, EmphasisStyle::Bold])
        ))
    );
    assert_eq!(
        emphasis("123***test***"),
        Ok((
            "",
            (
                "123",
                "test",
                vec![EmphasisStyle::Italic, EmphasisStyle::Bold]
            )
        ))
    );
}

pub fn parse_emphasis(input: &str) -> IResult<&str, (Inline, Inline)> {
    map(emphasis, |(leading, content, styles)| {
        (
            Inline::Text(text(leading)),
            Inline::Emphasis(Emphasis {
                child: text(content),
                styles,
            }),
        )
    })(input)
}
