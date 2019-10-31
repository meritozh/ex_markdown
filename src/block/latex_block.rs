use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    error::{context, ErrorKind},
    sequence::{preceded, terminated, tuple},
    Err, IResult,
};

use crate::token::{Block, LatexBlock};

fn latex_block<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    // TODO: Change implementation
    let content = move |input: &'a str| {
        let ending = tuple::<_, _, ((&str, ErrorKind)), _>((line_ending, tag("$$"), line_ending));
        let mut iter = input.chars().enumerate();
        loop {
            match iter.next() {
                Some((nth, '$')) if nth > 1 => {
                    if let Ok((_, (o1, o2, o3))) = ending(&input[(nth - 1)..]) {
                        let after = nth - 1 + o1.len() + o2.len() + o3.len();
                        let before = nth - 1;
                        return Ok((&input[after..], &input[..before]));
                    }
                }
                None => return Err(Err::Error((input, ErrorKind::TakeTill1))),
                _ => {}
            }
        }
    };
    context(
        "latex_block",
        preceded(terminated(tag("$$"), line_ending), content),
    )(input)
}

#[test]
fn latex_block_test() {
    assert_eq!(latex_block("$$\nlatex here\n$$\n"), Ok(("", "latex here")));
}

pub fn parse_latex_block(input: &str) -> IResult<&str, Block> {
    map(latex_block, |content| {
        Block::LatexBlock(LatexBlock { content })
    })(input)
}
