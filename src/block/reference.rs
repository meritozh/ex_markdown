use nom::{
    bytes::complete::tag,
    character::complete::{anychar, line_ending, not_line_ending},
    combinator::map,
    multi::many_till,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::token::{Block, RefDetail};

fn reference(input: &str) -> IResult<&str, (&str, &str)> {
    preceded(
        tag("["),
        tuple((
            map(many_till(anychar, tag("]: ")), |(matched, _)| {
                &input[1..=matched.len()]
            }),
            not_line_ending,
        )),
    )(input)
}

#[test]
fn reference_test() {
    assert_eq!(
        reference("[ref]: ref detail here."),
        Ok(("", ("ref", "ref detail here.")))
    );
}

pub fn parse_reference(input: &str) -> IResult<&str, Block> {
    map(
        terminated(reference, line_ending),
        |(reference, content)| Block::RefDetail(RefDetail { reference, content }),
    )(input)
}
