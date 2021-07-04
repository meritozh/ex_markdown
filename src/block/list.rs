use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, not_line_ending, space1},
    combinator::{map, map_parser, map_res, rest, value},
    error::context,
    multi::many0_count,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

use crate::token::{Block, List, ListStyle};

fn bullet_list(input: &str) -> IResult<&str, (usize, &str)> {
    context(
        "bullet list",
        map_parser(
            terminated(not_line_ending, line_ending),
            separated_pair(
                map(many0_count(char(' ')), |c| c / 2),
                tuple((char('-'), space1)),
                rest,
            ),
        ),
    )(input)
}

fn number_list(input: &str) -> IResult<&str, ((usize, u8), &str)> {
    context(
        "number list",
        map_parser(
            terminated(not_line_ending, line_ending),
            separated_pair(
                tuple((
                    map(many0_count(char(' ')), |c| c / 2),
                    map_res(digit1, |s: &str| s.parse::<u8>()),
                )),
                tuple((char('.'), space1)),
                rest,
            ),
        ),
    )(input)
}

fn task_list(input: &str) -> IResult<&str, ((usize, bool), &str)> {
    context(
        "task list",
        map_parser(
            terminated(not_line_ending, line_ending),
            separated_pair(
                separated_pair(
                    map(many0_count(char(' ')), |c| c / 2),
                    tag("- "),
                    alt((value(false, tag("[ ]")), value(true, tag("[x]")))),
                ),
                space1,
                rest,
            ),
        ),
    )(input)
}

#[test]
fn list_test() {
    assert_eq!(bullet_list("- 123\n"), Ok(("", (0, "123"))));
    assert_eq!(bullet_list("  - 123\n"), Ok(("", (1, "123"))));
    assert_eq!(number_list("1. asd\n"), Ok(("", ((0, 1), "asd"))));
    assert_eq!(task_list("- [ ] task\n"), Ok(("", ((0, false), "task"))));
}

pub(super) fn parse_list(input: &str) -> IResult<&str, Block> {
    alt((
        map(task_list, |((level, checked), content)| {
            Block::List(List {
                style: ListStyle::Task(checked),
                level,
                content,
                ..Default::default()
            })
        }),
        map(bullet_list, |(level, content)| {
            Block::List(List {
                style: ListStyle::Bullet,
                level,
                content,
                ..Default::default()
            })
        }),
        map(number_list, |((level, digit), content)| {
            Block::List(List {
                style: ListStyle::Number(digit),
                level,
                content,
                ..Default::default()
            })
        }),
    ))(input)
}
