use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0},
    combinator::{map, map_parser},
    error::{context, ErrorKind},
    sequence::{preceded, terminated, tuple},
    Err, IResult,
};

use crate::token::{Block, Command};

fn command<'a>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    let content = move |input: &'a str| {
        let ending = tuple::<_, _, ((&str, ErrorKind)), _>((line_ending, tag("}}"), line_ending));
        let mut iter = input.chars().enumerate();
        loop {
            match iter.next() {
                Some((nth, '}')) if nth > 1 => {
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
        "command",
        tuple((
            preceded(
                tag("{{"),
                terminated(
                    map_parser(not_line_ending, preceded(space0, not_line_ending)),
                    line_ending,
                ),
            ),
            content,
        )),
    )(input)
}

#[test]
fn command_test() {
    assert_eq!(
        command("{{ chart\nchart info here\n}}\n"),
        Ok(("", ("chart", "chart info here")))
    );
}

pub fn parse_command(input: &str) -> IResult<&str, Block> {
    map(command, |(tag, content)| {
        Block::Command(Command { tag, content })
    })(input)
}
