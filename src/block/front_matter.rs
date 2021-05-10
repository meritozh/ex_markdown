use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, not_line_ending, space0},
    combinator::map,
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::token::{Block, FrontMatter};

#[cfg(test)]
use nom::{
    error::{Error, ErrorKind},
    Err,
};

pub fn front_matter(input: &str) -> IResult<&str, &str> {
    context(
        "front matter",
        preceded(
            tuple((tag("---"), line_ending)),
            terminated(take_until("---"), tuple((tag("---"), line_ending))),
        ),
    )(input)
}

#[test]
fn front_matter_test() {
    assert_eq!(front_matter("---\n123\n---\nabc"), Ok(("abc", "123\n")));
    assert_eq!(
        front_matter("---\n123\ntest\n - lalala\n---\n"),
        Ok(("", "123\ntest\n - lalala\n"))
    );
    assert_eq!(
        front_matter("123"),
        Err(Err::Error(Error::new("123", ErrorKind::Tag)))
    );
}

pub fn parse_front_matter(input: &str) -> IResult<&str, Block> {
    map(front_matter, |child| {
        Block::FrontMatter(FrontMatter { child })
    })(input)
}
