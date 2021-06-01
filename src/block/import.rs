use nom::{
    bytes::streaming::tag,
    character::{complete::line_ending, streaming::not_line_ending},
    combinator::map,
    error::context,
    sequence::delimited,
    IResult,
};

use crate::token::{Block, Import};

fn import(input: &str) -> IResult<&str, &str> {
    context(
        "import",
        delimited(tag("<<< @"), not_line_ending, line_ending),
    )(input)
}

#[test]
fn import_test() {
    assert_eq!(import("<<< @/filepath\n"), Ok(("", "/filepath")));
}

pub(super) fn parse_import(input: &str) -> IResult<&str, Block> {
    map(import, |path| Block::Import(Import { path }))(input)
}
