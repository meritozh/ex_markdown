use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, space1},
    combinator::{eof, map, map_parser, recognize},
    error::{context, Error, ErrorKind},
    multi::{count, many1},
    sequence::{delimited, pair, preceded, tuple},
    Err, IResult, Offset, Slice,
};

use crate::token::{Image, Inline};

use super::shared::delimiter::{Delimiter, DelimiterStack, DelimiterType};

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
    let (remain, content) = delimited(char('('), take_until(")"), char(')'))(input)?;
    let len = content.split_whitespace().collect::<Vec<_>>().len();
    match len {
        1 => Ok((remain, (content, None))),
        2 => pair(
            map_parser(take_until(" "), destionation),
            preceded(space1, title),
        )(content)
        .map(|r| (remain, r.1)),
        _ => Err(Err::Error(Error::new(input, ErrorKind::TakeUntil))),
    }
}

fn text(input: &str, skip: usize) -> IResult<&str, &str> {
    let skip = count(preceded(take_until("]"), char(']')), skip)(input)?;
    let (remain, _) = take_until("]")(skip.0)?;
    char(']')(remain).map(|r| {
        let offset = input.offset(remain);
        (r.0, input.slice(..offset))
    })
}

fn open_bracket(input: &str) -> IResult<&str, DelimiterType> {
    alt((
        map(preceded(take_until("!["), tag("![")), |_| {
            DelimiterType::MarkOpenBracket
        }),
        map(preceded(take_until("["), char('[')), |_| {
            DelimiterType::OpenBracket
        }),
    ))(input)
}

fn close_bracket(input: &str) -> IResult<&str, ()> {
    preceded(take_until("]"), char(']'))(input).map(|r| (r.0, ()))
}

fn stack(input: &str) -> IResult<&str, (&str, (&str, Option<&str>))> {
    let mut inner_close_bracket = 0;
    let mut stack = DelimiterStack::default();
    let mut i = input.clone();

    while eof::<_, Error<&str>>(i).is_err() {
        if let Ok((o, t)) = open_bracket(i) {
            stack.0.push(Delimiter {
                delimiter: t,
                slice: o,
            });
            i = o;
            continue;
        } else if let Ok((remain, _)) = close_bracket(i) {
            if let Some(e) = stack.0.pop() {
                match e.delimiter {
                    DelimiterType::MarkOpenBracket => {
                        let res = tuple((|i| text(i, inner_close_bracket), destination_and_title))(
                            e.slice,
                        );
                        if res.is_ok() {
                            return res;
                        } else {
                            inner_close_bracket += 1;
                            i = remain;
                        }
                    }
                    DelimiterType::OpenBracket => {
                        inner_close_bracket += 1;
                        i = remain;
                    }
                    _ => unreachable!(),
                }
                continue;
            }
        }
        break;
    }

    Err(Err::Error(Error::new(input, ErrorKind::Eof)))
}

fn image(input: &str) -> IResult<&str, (&str, (&str, Option<&str>))> {
    context("image", stack)(input)
}

#[test]
fn image_test() {
    assert_eq!(image("![test](url)"), Ok(("", ("test", ("url", None)))));
    assert_eq!(image("![![test](url)"), Ok(("", ("test", ("url", None)))));
    assert_eq!(image("![[test]](url)"), Ok(("", ("[test]", ("url", None)))));
    assert_eq!(image("[![test](url)]"), Ok(("]", ("test", ("url", None)))));
    assert_eq!(
        image("[![test](url 'title')]"),
        Ok(("]", ("test", ("url", Some("title")))))
    );
    assert_eq!(
        image("[![test](url)](url)"),
        Ok(("](url)", ("test", ("url", None))))
    );

    assert_eq!(
        image("![test]](url)"),
        Err(Err::Error(Error::new("![test]](url)", ErrorKind::Eof)))
    );
    assert_eq!(
        image("![![test](url title)]"),
        Err(Err::Error(Error::new(
            "![![test](url title)]",
            ErrorKind::Eof
        )))
    );
}

pub(crate) fn parse_image(input: &str) -> IResult<&str, Inline> {
    map(image, |(label, (url, title))| {
        Inline::Image(Image { label, url, title })
    })(input)
}
