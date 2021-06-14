use nom::{
    character::complete::anychar,
    combinator::{eof, map, peek},
    error::context,
    sequence::preceded,
    IResult,
};

#[cfg(test)]
use nom::{
    error::{Error, ErrorKind},
    Err,
};

use crate::{
    token::{Inline, Text},
    utils::nom_extend::take_until_parser_matches_and_consume,
};

pub fn text(input: &str) -> IResult<&str, &str> {
    context(
        "text",
        preceded(peek(anychar), take_until_parser_matches_and_consume(eof)),
    )(input)
}

#[test]
fn text_test() {
    assert_eq!(text("asd"), Ok(("", "asd")));
    assert_eq!(text(""), Err(Err::Error(Error::new("", ErrorKind::Eof))));
}

pub fn parse_text(input: &str) -> IResult<&str, Inline> {
    map(text, |content| Inline::Text(Text { content }))(input)
}
