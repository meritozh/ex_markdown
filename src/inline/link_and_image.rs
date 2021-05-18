use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::char,
    combinator::{eof, map},
    sequence::tuple,
    IResult,
};

use crate::token::{Inline, Link};

enum DelimiterType {
    // [
    OpenBracket,
    // ![
    MarkOpenBracket,
    // ]
    CloseBracket,
}

struct Delimiter<'a> {
    delimiter: DelimiterType,
    slice: &'a str,
    active: bool,
}

#[derive(Default)]
struct DelimiterStack<'a>(Vec<Delimiter<'a>>);

// fn emphasis(input: &str) -> IResult<&str, LinkDelimiterType> {
//     alt((
//         map(tuple((take_until("_"), char('_'))), |_| {
//             LinkDelimiterType::Underscore
//         }),
//         map(tuple((take_until("*"), char('*'))), |_| {
//             LinkDelimiterType::Asterisk
//         }),
//     ))(input)
// }

fn open_link_or_image(input: &str) -> IResult<&str, DelimiterType> {
    alt((
        map(take_until("!["), |_| DelimiterType::MarkOpenBracket),
        map(take_until("["), |_| DelimiterType::OpenBracket),
    ))(input)
}

fn close(input: &str) -> IResult<&str, DelimiterType> {
    map(tuple((take_until("]"), char(']'))), |_| {
        DelimiterType::CloseBracket
    })(input)
}

fn link_and_image(input: &str) -> IResult<&str, Inline> {
    let mut stack = DelimiterStack::default();
    let mut i = input.clone();

    while eof::<_, nom::error::Error<&str>>(i).is_err() {
        if let Ok((o, t)) = open_link_or_image(i) {
            stack.0.push(Delimiter {
                delimiter: t,
                slice: o,
                active: true,
            });
            i = o;
        } else if let Ok((o, t)) = close(i) {
            if let Some(e) = stack.0.pop() {
                match e.delimiter {
                    DelimiterType::OpenBracket => {
                        return Inline::Link(Link {
                            label: e.
                        });
                    }
                }
            }
        }
    }

    Ok((i, inlines))
}
