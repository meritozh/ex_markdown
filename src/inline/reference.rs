use nom::{
    bytes::complete::take_until, character::complete::char, combinator::map, error::context,
    sequence::delimited, IResult,
};

use crate::token::{Inline, Reference};

fn reference(input: &str) -> IResult<&str, &str> {
    context(
        "reference",
        delimited(char('['), take_until("]"), char(']')),
    )(input)
}

#[test]
fn reference_test() {
    assert_eq!(reference("[tag]"), Ok(("", "tag")));
    assert_eq!(reference("[^label]"), Ok(("", "^label")));
}

fn parse_reference(input: &str) -> IResult<&str, Inline> {
    map(reference, |label| Inline::Reference(Reference { label }))(input)
}
