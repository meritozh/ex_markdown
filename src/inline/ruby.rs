use nom::{
    bytes::complete::take_until,
    character::complete::char,
    combinator::{map, verify},
    error::context,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

use crate::token::{Inline, Ruby};

fn ruby(input: &str) -> IResult<&str, (&str, &str)> {
    context(
        "ruby",
        delimited(
            char('{'),
            map(
                verify(
                    separated_list1(char('|'), take_until("|")),
                    |v: &Vec<&str>| v.len() == 2,
                ),
                |v: Vec<&str>| 
                // SAFETY: checked length in verify
                unsafe { (*v.get_unchecked(0), *v.get_unchecked(1)) },
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
            annotation
        })
    })(input)
}
