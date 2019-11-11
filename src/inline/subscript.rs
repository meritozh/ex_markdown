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
    inline::text::text,
    token::{Inline, Subscript},
};

fn subscript(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "subscript",
        map(
            verify(
                tuple((
                    map(
                        many_till(anychar, many1_count(tag("~"))),
                        |(leading, count)| (leading.len(), count),
                    ),
                    many_till(anychar, many1_count(tag("~"))),
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
fn subscript_test() {
    assert_eq!(subscript("~test~"), Ok(("", ("", "test"))));
    assert_eq!(
        subscript("~~test~~"),
        Err(Err::Error(("~~test~~", ErrorKind::Verify)))
    );
}

pub fn parse_subscript(input: &str) -> IResult<&str, Vec<Inline>> {
    map(subscript, |(leading, content)| {
        vec![
            Inline::Text(text(leading)),
            Inline::Subscript(Subscript {
                child: text(content),
            }),
        ]
    })(input)
}
