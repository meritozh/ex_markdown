use nom::{
    branch::alt,
    character::complete::{line_ending, space1},
    combinator::{map, value},
    multi::many1,
    sequence::preceded,
    IResult,
};

use crate::token::Block;

#[cfg(test)]
use nom::{error::ErrorKind, Err};

/// Recognize `(#+)<space><text>\n` style markdown input.
fn blank_line(input: &str) -> IResult<&str, &str> {
    alt((
        value("", many1(line_ending)),
        preceded(space1, value("", many1(line_ending))),
    ))(input)
}

#[test]
fn blank_line_test() {
    assert_eq!(blank_line("\n"), Ok(("", "")));
    assert_eq!(blank_line("\t\n"), Ok(("", "")));
    assert_eq!(blank_line("  \n"), Ok(("", "")));
    assert_eq!(blank_line("  \t\n"), Ok(("", "")));
    assert_eq!(blank_line("  \t\n\n\n\n"), Ok(("", "")));
    assert_eq!(blank_line("\n\n\n\n"), Ok(("", "")));
    assert_eq!(
        blank_line("123\n"),
        Err(Err::Error(("123\n", ErrorKind::Space)))
    );
}

pub fn parse_blank_line(input: &str) -> IResult<&str, Block> {
    map(blank_line, |_| Block::BlankLine)(input)
}
