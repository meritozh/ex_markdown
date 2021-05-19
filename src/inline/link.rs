use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::{anychar, char, space0},
    combinator::{eof, map, map_parser, recognize, rest},
    error::{context, Error, ErrorKind},
    multi::many1,
    sequence::{delimited, pair, preceded, tuple},
    Err, IResult,
};

use crate::token::{Inline, Link};

use super::shared::delimiter::{close, Delimiter, DelimiterStack, DelimiterType};

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

// TODO: support uri escape
fn destionation(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(char('<'), take_until(">"), char('>')),
        recognize(many1(anychar)),
    ))(input)
}

fn destination_and_title(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    // TODO: support balanced parentheses
    let (_, content) = preceded(char('('), take_until(")"))(input)?;
    let v: Vec<&str> = content.split_whitespace().collect();
    match v.len() {
        1 => Ok((
            "",
            (
                // SAFETY: checked v.len()
                unsafe { *v.get_unchecked(0) },
                None,
            ),
        )),
        2 => pair(
            map_parser(take_until(" "), destionation),
            preceded(space0, title),
        )(input),
        _ => Err(Err::Error(Error::new("123\n", ErrorKind::TakeUntil))),
    }
}

fn text(input: &str) -> IResult<&str, &str> {
    delimited(char('['), take_until("]"), char(']'))(input)
}

fn open_link(input: &str) -> IResult<&str, DelimiterType> {
    map(take_until("["), |_| DelimiterType::MarkOpenBracket)(input)
}

fn link(input: &str) -> IResult<&str, (&str, (&str, Option<&str>))> {
    context("link", tuple((text, destination_and_title)))(input)
}

pub(crate) fn parse_link(input: &str) -> IResult<&str, Inline> {
    let mut stack = DelimiterStack::default();
    let mut i = input.clone();

    while eof::<_, nom::error::Error<&str>>(i).is_err() {
        if let Ok((o, t)) = open_link(i) {
            stack.0.push(Delimiter {
                delimiter: t,
                slice: o,
                active: true,
            });
            // consume "![", then unwrap it, open_link guarantee safety.
            i = preceded(char('['), rest::<_, (_, ErrorKind)>)(o).unwrap().0;
        } else if close(i).is_ok() {
            if let Some(e) = stack.0.pop() {
                match e.delimiter {
                    DelimiterType::OpenBracket => {
                        return map(link, |(label, (url, title))| {
                            Inline::Link(Link { label, url, title })
                        })(e.slice);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    Err(Err::Error(Error::new(input, ErrorKind::Eof)))
}
