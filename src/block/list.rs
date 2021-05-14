use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, not_line_ending, space1},
    combinator::{map, map_parser, value},
    error::context,
    multi::many0_count,
    sequence::{terminated, tuple},
    IResult,
};

use crate::token::{Block, List, ListStyle};

fn bullet_list(input: &str) -> IResult<&str, (usize, &str)> {
    context(
        "bullet list",
        map_parser(terminated(not_line_ending, line_ending), |content| {
            tuple((many0_count(char(' ')), char('-'), space1))(content)
                .map(|(content, (level, _, _))| ("", (level, content)))
        }),
    )(input)
}

fn number_list<'a>(input: &'a str) -> IResult<&'a str, (usize, u32, &'a str)> {
    context(
        "number list",
        map_parser(
            terminated(not_line_ending, line_ending),
            |content: &'a str| {
                tuple((many0_count(char(' ')), digit1, char('.'), space1))(content).map(
                    |(content, (count, digit, _, _))| {
                        ("", (count, digit.parse::<u32>().unwrap(), content))
                    },
                )
            },
        ),
    )(input)
}

fn task_list(input: &str) -> IResult<&str, (usize, bool, &str)> {
    context(
        "task list",
        map_parser(terminated(not_line_ending, line_ending), |content| {
            tuple((
                many0_count(char(' ')),
                terminated(
                    alt((value(false, tag("- [ ]")), value(true, tag("- [x]")))),
                    space1,
                ),
            ))(content)
            .map(|(content, (count, checked))| ("", (count, checked, content)))
        }),
    )(input)
}

#[test]
fn list_test() {
    assert_eq!(bullet_list("- 123\n"), Ok(("\n", (0, "123"))));
    assert_eq!(bullet_list("  - 123\n"), Ok(("\n", (2, "123"))));
    assert_eq!(number_list("1. asd"), Ok(("", (0, 1, "asd"))));
    assert_eq!(task_list("- [ ] task"), Ok(("", (0, false, "task"))));
}

pub fn parse_list(input: &str) -> IResult<&str, Block> {
    alt((
        map(task_list, |(level, checked, content)| {
            Block::List(List {
                style: ListStyle::Task(checked),
                level,
                content,
            })
        }),
        map(bullet_list, |(level, content)| {
            Block::List(List {
                style: ListStyle::Bullet,
                level,
                content,
            })
        }),
        map(number_list, |(level, digit, content)| {
            Block::List(List {
                style: ListStyle::Number(digit),
                level,
                content,
            })
        }),
    ))(input)
}
