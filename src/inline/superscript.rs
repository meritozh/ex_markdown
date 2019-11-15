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
    token::{Inline, Superscript},
};

fn superscript(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "superscript",
        map(
            verify(
                tuple((
                    map(
                        many_till(anychar, many1_count(tag("^"))),
                        |(leading, count)| (leading.len(), count),
                    ),
                    many_till(anychar, many1_count(tag("^"))),
                )),
                |((_, count1), (_, count2))| *count1 == 1 && *count2 == 1,
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
fn superscript_test() {
    assert_eq!(superscript("^test^"), Ok(("", ("", "test"))));
    assert_eq!(
        superscript("^^test^^"),
        Err(Err::Error(("^^test^^", ErrorKind::Verify)))
    );
}

pub fn parse_superscript(input: &str) -> IResult<&str, Vec<Inline>> {
    map(superscript, |(leading, content)| {
        vec![
            parse_inline(leading),
            vec![Inline::Superscript(Superscript {
                child: text(content),
            })],
        ]
        .into_iter()
        .flatten()
        .collect()
    })(input)
}
