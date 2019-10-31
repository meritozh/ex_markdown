use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, not_line_ending},
    combinator::{map, map_parser, value},
    error::context,
    multi::many0_count,
    sequence::tuple,
    IResult,
};

use crate::token::{Block, List, ListStyle};

fn bullet_list(input: &str) -> IResult<&str, (&str, usize)> {
    context(
        "bullet_list",
        map_parser(not_line_ending, |content| {
            tuple((many0_count(char(' ')), tag("- ")))(content)
                .map(|(remain, (count, _))| ("", (remain, count)))
        }),
    )(input)
}

fn number_list(input: &str) -> IResult<&str, (&str, u32, usize)> {
    context(
        "number_list",
        map_parser(not_line_ending, |content: &str| {
            tuple((many0_count(char(' ')), digit1, tag(". ")))(content).map(
                |(remain, (count, digit, _))| ("", (remain, digit.parse::<u32>().unwrap(), count)),
            )
        }),
    )(input)
}

fn task_list(input: &str) -> IResult<&str, (&str, usize, bool)> {
    context(
        "task_list",
        map_parser(not_line_ending, |content| {
            tuple((
                many0_count(char(' ')),
                alt((value(false, tag("- [ ] ")), value(true, tag("- [x] ")))),
            ))(content)
            .map(|(remain, (count, checked))| ("", (remain, count, checked)))
        }),
    )(input)
}

#[test]
fn list_test() {
    assert_eq!(bullet_list("- 123\n"), Ok(("\n", ("123", 0))));
    assert_eq!(bullet_list("  - 123\n"), Ok(("\n", ("123", 2))));
    assert_eq!(number_list("1. asd"), Ok(("", ("asd", 1, 0))));
    assert_eq!(task_list("- [ ] task"), Ok(("", ("task", 0, false))));
}

pub fn parse_list(input: &str) -> IResult<&str, Block> {
    alt((
        map(task_list, |(content, indentation, checked)| {
            Block::List(List {
                style: ListStyle::Task(checked),
                indentation,
                content,
            })
        }),
        map(bullet_list, |(content, indentation)| {
            Block::List(List {
                style: ListStyle::Bullet,
                indentation,
                content,
            })
        }),
        map(number_list, |(content, digit, indentation)| {
            Block::List(List {
                style: ListStyle::Number(digit),
                indentation,
                content,
            })
        }),
    ))(input)
}
