use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, space0},
    combinator::{map, map_parser},
    error::{context, Error, ErrorKind},
    sequence::{pair, preceded, tuple},
    Err, IResult,
};

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

fn content(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    let v: Vec<&str> = input.split_whitespace().collect();
    match v.len() {
        1 => Ok((
            "",
            (
                // SAFETY: checked v.len()
                unsafe { *v.get_unchecked(0) },
                None,
            ),
        )),
        2 => pair(take_until(" "), preceded(space0, title))(input),
        _ => Err(Err::Error(Error::new("123\n", ErrorKind::TakeUntil))),
    }
}

fn link(input: &str) -> IResult<&str, (&str, (&str, Option<&str>))> {
    context(
        "link",
        tuple((
            preceded(char('['), take_until("]")),
            map_parser(preceded(tag("]("), take_until(")")), content),
        )),
    )(input)
}
