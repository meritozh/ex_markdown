use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, map_parser, peek},
    error::ErrorKind,
    multi::{count, many1, many1_count, many_till},
    sequence::tuple,
    IResult,
};

use std::cmp;

use crate::{
    inline::text::text,
    token::{Emphasis, EmphasisStyle, Inline},
};

fn emphasis(input: &str) -> IResult<&str, (&str, &str, Vec<EmphasisStyle>)> {
    map(
        tuple((
            many1_count(tag("*")),
            peek(map(
                many_till(anychar, many1_count(tag("*"))),
                |(vec, count)| (vec.len(), count),
            )),
        )),
        |(count1, (count2, count3))| {
            let delimited = cmp::min(count1, count3);
            let leading = if count1 <= delimited {
                ""
            } else {
                &input[..count1 - delimited]
            };
            let styles = match delimited {
                1 => vec![EmphasisStyle::Italic],
                _ if delimited % 2 == 0 => vec![EmphasisStyle::Bold],
                _ if delimited % 2 == 1 => vec![EmphasisStyle::Italic, EmphasisStyle::Bold],
                _ => unreachable!(),
            };
            (
                delimited + count2,
                leading,
                &input[count1..count1 + count2],
                styles,
            )
        },
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
}

pub fn parse_emphasis(input: &str) -> IResult<&str, Inline> {
    map(emphasis, |(leading, content, styles)| {
        Inline::Emphasis(Emphasis {
            leading: text(leading),
            child: text(content),
            styles,
        })
    })(input)
}
