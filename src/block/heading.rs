use nom::{
    character::complete::{char, line_ending, not_line_ending, space1},
    combinator::{map, verify},
    error::context,
    multi::many_m_n,
    sequence::{separated_pair, terminated},
    IResult,
};

#[cfg(test)]
use nom::{
    error::{Error, ErrorKind},
    Err,
};

use crate::token::{Block, Heading};

fn heading(input: &str) -> IResult<&str, (usize, &str)> {
    context(
        "heading",
        separated_pair(
            map(many_m_n(1, 6, char('#')), |matched| matched.len()),
            space1,
            verify(terminated(not_line_ending, line_ending), |heading: &str| {
                !heading.is_empty()
            }),
        ),
    )(input)
}

#[test]
fn heading_test() {
    assert_eq!(heading("### 123\n"), Ok(("", (3, "123"))));
    assert_eq!(heading("### ### 123\n"), Ok(("", (3, "### 123"))));
    assert_eq!(heading("###### 123\nabc"), Ok(("abc", (6, "123"))));
    assert_eq!(
        heading("###"),
        Err(Err::Error(Error::new("", ErrorKind::Char)))
    );
    assert_eq!(
        heading("### "),
        Err(Err::Error(Error::new("", ErrorKind::Verify)))
    );
    assert_eq!(
        heading("######123"),
        Err(Err::Error(Error::new("123", ErrorKind::Char)))
    );
    assert_eq!(
        heading("####### 123"),
        Err(Err::Error(Error::new("####### 123", ErrorKind::Verify)))
    );
}

pub fn parse_heading(input: &str) -> IResult<&str, Block> {
    map(heading, |(level, content)| {
        Block::Heading(Heading { level, content })
    })(input)
}
