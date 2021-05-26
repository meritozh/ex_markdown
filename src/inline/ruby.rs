use nom::{
    bytes::complete::take_until,
    character::complete::char,
    combinator::{map, map_parser, rest, verify},
    error::context,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::token::{Inline, Ruby};

fn ruby(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "ruby",
        delimited(
            char('{'),
            map_parser(
                take_until("}"),
                tuple((take_until("|"), preceded(char('|'), rest))),
            ),
            char('}'),
        ),
    )(input)
}

#[test]
fn ruby_test() {
    assert_eq!(ruby("{test1|anno}"), Ok(("", ("test1", "anno"))));
}

pub fn parse_ruby(input: &str) -> IResult<&str, Inline> {
    map(ruby, |(content, annotation)| {
        Inline::Ruby(Ruby {
            content,
            annotation,
        })
    })(input)
}
