use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, verify},
    error::context,
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult,
};

use crate::{
    inline::parse_inline,
    token::{Inline, Mark},
};

fn mark(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "mark",
        map(
            verify(
                tuple((
                    map(
                        many_till(anychar, many1_count(tag("="))),
                        |(leading, count)| (leading.len(), count),
                    ),
                    many_till(anychar, many1_count(tag("="))),
                )),
                |((_, count1), (_, count2))| *count1 >= 2 && *count2 >= 2 && count1 == count2,
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
fn mark_test() {
    assert_eq!(mark("==test=="), Ok(("", ("", "test"))));
    assert_eq!(mark("123==test=="), Ok(("", ("123", "test"))));
}

pub fn parse_mark(input: &str) -> IResult<&str, Vec<Inline>> {
    map(mark, |(leading, mark)| {
        vec![
            parse_inline(leading),
            vec![Inline::Mark(Mark {
                children: parse_inline(mark),
            })],
        ]
        .into_iter()
        .flatten()
        .collect()
    })(input)
}
