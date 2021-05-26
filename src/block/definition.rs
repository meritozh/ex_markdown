use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, line_ending, not_line_ending, space1},
    combinator::{map, map_parser, not, peek, recognize, rest},
    error::{context, Error, ErrorKind},
    multi::many1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    Err, IResult,
};

use crate::token::{Block, Definition};

fn title(input: &str) -> IResult<&str, Option<&str>> {
    map(
        alt((
            preceded(char('\''), take_until("'")),
            preceded(char('"'), take_until("\"")),
        )),
        |o: &str| match o.len() {
            0 => None,
            _ => Some(o),
        },
    )(input)
}

fn destionation(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(char('<'), take_until(">"), char('>')),
        recognize(many1(anychar)),
    ))(input)
}

fn destination_and_title(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    let len = input.split_whitespace().collect::<Vec<_>>().len();
    match len {
        1 => Ok(("", (input, None))),
        2 => pair(
            map_parser(take_until(" "), destionation),
            preceded(space1, title),
        )(input),
        _ => Err(Err::Error(Error::new(input, ErrorKind::TakeUntil))),
    }
}

fn definition(input: &str) -> IResult<&str, (&str, (&str, Option<&str>))> {
    context(
        "definition",
        tuple((
            delimited(
                char('['),
                map_parser(take_until("]:"), preceded(peek(not(char('^'))), rest)),
                tuple((tag("]:"), space1)),
            ),
            map_parser(
                terminated(not_line_ending, line_ending),
                destination_and_title,
            ),
        )),
    )(input)
}

#[test]
fn definition_test() {
    assert_eq!(
        definition("[ref]: url 'title'\n"),
        Ok(("", ("ref", ("url", Some("title")))))
    );
    assert_eq!(definition("[ref]: url\n"), Ok(("", ("ref", ("url", None)))));
}

pub fn parse_definition(input: &str) -> IResult<&str, Block> {
    map(definition, |(label, (url, title))| {
        Block::Definition(Definition { label, url, title })
    })(input)
}
