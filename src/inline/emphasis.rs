use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete::{anychar, char},
    combinator::{flat_map, map, peek},
    error::{context, ParseError},
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult, Parser,
};

use std::cmp::min;

use crate::token::{Emphasis, EmphasisStyle, Inline};

fn factory<'a, E: ParseError<&'a str>>(
    symbol: char,
) -> impl Parser<&'a str, (usize, usize, usize), E> {
    map(
        tuple((
            many1_count(char(symbol)),
            many_till(anychar, peek(many1_count(char(symbol)))),
        )),
        |(left, (content, right))| (left, content.len(), right),
    )
}

fn emphasis(input: &str) -> IResult<&str, (Option<&str>, &str, EmphasisStyle)> {
    context(
        "emphsis",
        flat_map(
            map(
                alt((factory('*'), factory('_'))),
                |(left, content, right)| {
                    let style = match min(left, right) {
                        1 => EmphasisStyle::Italic,
                        i if i % 2 == 0 => EmphasisStyle::Bold,
                        _ => EmphasisStyle::BoldItalic,
                    };

                    match left - right {
                        0 => (left, (None, &input[left..left + content], style)),
                        i if i < 0 => (left, (None, &input[left..left + content], style)),
                        i if i > 0 => (
                            right,
                            (Some(&input[..i]), &input[left..left + content], style),
                        ),
                    }
                },
            ),
            |(count, result)| map(take(count), |_| result),
        ),
    )(input)
}

#[test]
fn emphasis_test() {
    assert_eq!(
        emphasis("**test**"),
        Ok(("", (None, "test", EmphasisStyle::Bold)))
    );
    assert_eq!(
        emphasis("***test**"),
        Ok(("", (Some("*"), "test", EmphasisStyle::Bold)))
    );
    assert_eq!(
        emphasis("**test***"),
        Ok(("*", (None, "test", EmphasisStyle::Bold)))
    );
    assert_eq!(
        emphasis("***test***"),
        Ok(("", (None, "test", EmphasisStyle::BoldItalic)))
    );
    assert_eq!(
        emphasis("123****test***"),
        Ok(("", (Some("123*"), "test", EmphasisStyle::BoldItalic)))
    );
}

pub fn parse_emphasis(input: &str) -> IResult<&str, Inline> {
    map(emphasis, |(leading, content, style)| {
        Inline::Emphasis(Emphasis {
            leading,
            content,
            style,
        })
    })(input)
}
