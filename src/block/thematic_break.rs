use nom::{
    bytes::complete::take_while,
    character::complete::{line_ending, not_line_ending},
    combinator::{map, verify},
    error::{context, ErrorKind},
    sequence::tuple,
    IResult,
};

#[cfg(test)]
use nom::Err;

use crate::token::Block;

fn thematic_break(input: &str) -> IResult<&str, &str> {
    context(
        "thematic_break",
        verify(not_line_ending, |content: &str| {
            let ret = take_while::<_, _, (_, ErrorKind)>(|ch| ch == '-')(content);
            if let Ok((remain, consumed)) = ret {
                if remain.is_empty() && consumed.len() >= 3 {
                    return true;
                }
            }
            false
        }),
    )(input)
}

#[test]
fn thematic_break_test() {
    assert_eq!(thematic_break("---"), Ok(("", "---")));
    assert_eq!(thematic_break("---------"), Ok(("", "---------")));
    assert_eq!(
        thematic_break(" ---"),
        Err(Err::Error((" ---", ErrorKind::Verify)))
    );
    assert_eq!(
        thematic_break("--"),
        Err(Err::Error(("--", ErrorKind::Verify)))
    );
}

pub fn parse_thematic_break(input: &str) -> IResult<&str, Block> {
    map(tuple((thematic_break, line_ending)), |_| {
        Block::ThematicBreak
    })(input)
}
