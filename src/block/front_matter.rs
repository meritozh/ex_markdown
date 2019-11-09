use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    error::{context, ErrorKind},
    sequence::{preceded, terminated, tuple},
    Err, IResult,
};

use crate::token::{Block, FrontMatter};

/// Match front matter:
///
/// ```yaml
/// ---
/// key1: value1
/// key2:
///   - value21
///   - value22
/// ---
/// ```
pub fn front_matter<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    // TODO: Change implementation
    let content = move |input: &'a str| {
        let ending = tuple::<_, _, (&str, ErrorKind), _>((line_ending, tag("---"), line_ending));
        let mut iter = input.chars().enumerate();
        loop {
            match iter.next() {
                Some((nth, '-')) if nth > 1 => {
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
        "front_matter",
        preceded(terminated(tag("---"), line_ending), content),
    )(input)
}

#[test]
fn front_matter_test() {
    assert_eq!(front_matter("---\n123\n---\nabc"), Ok(("abc", "123")));
    assert_eq!(
        front_matter("---\n123\ntest\n - lalala\n---\n"),
        Ok(("", "123\ntest\n - lalala"))
    );
    assert_eq!(
        front_matter("123"),
        Err(Err::Error(("123", ErrorKind::Tag)))
    );
}

pub fn parse_front_matter(input: &str) -> IResult<&str, Block> {
    map(front_matter, |content| {
        Block::FrontMatter(FrontMatter { child: content })
    })(input)
}
