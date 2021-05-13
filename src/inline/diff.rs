use nom::{
    branch::alt,
    character::complete::{anychar, char},
    combinator::{map, verify},
    error::{context, ParseError},
    multi::{many1_count, many_till},
    sequence::tuple,
    IResult, Parser,
};

use crate::token::{Diff, DiffStyle, Inline};

fn factory<'a, E: ParseError<&'a str>>(symbol: char) -> impl Parser<&'a str, (usize, usize), E> {
    map(
        verify(
            tuple((
                many1_count(char(symbol)),
                // TODO: anychar need convert line_ending to space.
                many_till(anychar, many1_count(char(symbol))),
            )),
            |(left, (_, right)): &(usize, (_, usize))| *left >= 2 && *right >= 2 && left == right,
        ),
        |(left, (content, _))| (left, content.len()),
    )
}

fn plus(input: &str) -> IResult<&str, (&str, DiffStyle)> {
    map(factory('+'), |(left, content)| {
        // TODO: consider use consume to construct output instead of manually string slice
        (&input[left..left + content], DiffStyle::Plus)
    })(input)
}

fn minus(input: &str) -> IResult<&str, (&str, DiffStyle)> {
    map(factory('-'), |(left, content)| {
        (&input[left..left + content], DiffStyle::Minus)
    })(input)
}

fn diff(input: &str) -> IResult<&str, (&str, DiffStyle)> {
    context("diff", alt((plus, minus)))(input)
}

#[test]
fn diff_test() {
    assert_eq!(diff("++test++"), Ok(("", ("test", DiffStyle::Plus))));
    assert_eq!(diff("--test--"), Ok(("", ("test", DiffStyle::Minus))));
}

pub fn parse_diff(input: &str) -> IResult<&str, Inline> {
    map(diff, |(content, style)| {
        Inline::Diff(Diff { style, content })
    })(input)
}
