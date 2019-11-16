use nom::{
    branch::alt,
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
    token::{Diff, DiffStyle, Inline},
};

fn factory<'a>(
    symbol: &'static str,
) -> impl Fn(&'a str) -> IResult<&'a str, (usize, usize, usize)> {
    map(
        verify(
            tuple((
                map(
                    many_till(anychar, many1_count(tag(symbol))),
                    |(leading, count)| (leading.len(), count),
                ),
                many_till(anychar, many1_count(tag(symbol))),
            )),
            |((_, count1), (_, count2))| *count1 >= 2 && *count2 >= 2 && count1 == count2,
        ),
        |((count1, count2), (content, _))| (count1, count2, content.len()),
    )
}

fn plus(input: &str) -> IResult<&str, (&str, &str, DiffStyle)> {
    map(factory("+"), |(count1, count2, count3)| {
        (
            &input[..count1],
            &input[count1 + count2..count1 + count2 + count3],
            DiffStyle::Plus,
        )
    })(input)
}

fn minus(input: &str) -> IResult<&str, (&str, &str, DiffStyle)> {
    map(factory("-"), |(count1, count2, count3)| {
        (
            &input[..count1],
            &input[count1 + count2..count1 + count2 + count3],
            DiffStyle::Minus,
        )
    })(input)
}

fn diff(input: &str) -> IResult<&str, (&str, &str, DiffStyle)> {
    context("diff", alt((plus, minus)))(input)
}

#[test]
fn diff_test() {
    assert_eq!(diff("++test++"), Ok(("", ("", "test", DiffStyle::Plus))));
    assert_eq!(diff("--test--"), Ok(("", ("", "test", DiffStyle::Minus))));
    assert_eq!(
        diff("123--test--"),
        Ok(("", ("123", "test", DiffStyle::Minus)))
    );
}

pub fn parse_diff(input: &str) -> IResult<&str, Vec<Inline>> {
    map(diff, |(leading, content, style)| {
        vec![
            parse_inline(leading),
            vec![Inline::Diff(Diff {
                children: parse_inline(content),
                style,
            })],
        ]
        .into_iter()
        .flatten()
        .collect()
    })(input)
}
