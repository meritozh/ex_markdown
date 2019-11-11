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
    token::{Inline, Latex},
};

fn latex(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "latex",
        map(
            verify(
                tuple((
                    map(
                        many_till(anychar, many1_count(tag("$"))),
                        |(leading, count)| (leading.len(), count),
                    ),
                    many_till(anychar, many1_count(tag("$"))),
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
fn latex_test() {
    assert_eq!(latex("$test$"), Ok(("", ("", "test"))));
    assert_eq!(latex("$$$test$$$"), Ok(("", ("", "test"))));
    assert_eq!(latex("123$$$test$$$"), Ok(("", ("123", "test"))));
    assert_eq!(
        latex("$$test$"),
        Err(Err::Error(("$$test$", ErrorKind::Verify)))
    );
    assert_eq!(
        latex("$test$$"),
        Err(Err::Error(("$test$$", ErrorKind::Verify)))
    );
    assert_eq!(latex("$test"), Err(Err::Error(("", ErrorKind::Eof))));
    assert_eq!(latex("$$"), Err(Err::Error(("", ErrorKind::Eof))));
}

pub fn parse_latex(input: &str) -> IResult<&str, Vec<Inline>> {
    map(latex, |(leading, content)| {
        vec![
            Inline::Text(text(leading)),
            Inline::Latex(Latex { content }),
        ]
    })(input)
}
