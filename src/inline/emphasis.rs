use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::{eof, map, not, peek},
    error::{context, Error, ErrorKind},
    multi::many1_count,
    sequence::{preceded, terminated},
    Err, IResult, Offset, Slice,
};

use crate::{
    token::{Emphasis, EmphasisStyle, Inline},
    utils::nom_extend::take_until_parser_matches,
};

use super::shared::delimiter::{
    decrease_delimiter_count, get_delimiter_associate_count, is_same_delimiter_type, Delimiter,
    DelimiterStack, DelimiterType,
};

type EmphasisStack<'a> = Vec<(&'a str, EmphasisStyle)>;

fn mark(input: &str) -> IResult<&str, char> {
    alt((char('*'), char('_')))(input)
}

fn left_flank(input: &str) -> IResult<&str, DelimiterType> {
    let i = take_until_parser_matches(mark)(input)?.0;
    let ch = mark(i)?.1;

    terminated(
        map(preceded(space0, many1_count(char(ch))), move |c| match ch {
            '*' => DelimiterType::Asterisk(c),
            '_' => DelimiterType::Underline(c),
            _ => unreachable!(),
        }),
        not(space1),
    )(input)
}

fn right_flank(input: &str) -> IResult<&str, DelimiterType> {
    let i = take_until_parser_matches(mark)(input)?.0;
    let ch = mark(i)?.1;

    map(
        terminated(many1_count(char(ch)), peek(space0)),
        move |c| match ch {
            '*' => DelimiterType::Asterisk(c),
            '_' => DelimiterType::Underline(c),
            _ => unreachable!(),
        },
    )(i)
}

fn count_to_emphasis(count: usize) -> EmphasisStyle {
    match count {
        1 => EmphasisStyle::ITALIC,
        x if x % 2 == 0 => EmphasisStyle::BOLD,
        x if x % 2 == 1 => EmphasisStyle::BOLDITALIC,
        _ => unreachable!(),
    }
}

// TODO: this is messy, need refactor
fn truncate_until_matched_delimiter<'a>(
    input: &'a str,
    o: &'a str,
    stack: &mut DelimiterStack<'a>,
    right_delimiter: &DelimiterType,
    index: usize,
) -> (&'a str, (&'a str, EmphasisStyle)) {
    // stack pop to index
    stack.0.truncate(stack.0.len() - index);
    // SAFETY: truncate guarantee stack is not empty
    let matched = stack.0.pop().unwrap();
    let s = matched.slice;

    // left and right frank count, messy logic start
    let left_count = get_delimiter_associate_count(&matched.delimiter);
    let right_count = get_delimiter_associate_count(&right_delimiter);

    match (left_count, right_count) {
        // left < right, so we need forward left (left - right)
        (x, y) if x < y => {
            let offset = s.offset(o) - (y - x);
            return (
                s.slice(offset..),
                (s.slice(..offset - x), count_to_emphasis(left_count)),
            );
        }
        // left == right, all thing is perfect
        (x, y) if x == y => {
            let offset = s.offset(o);
            return (
                s.slice(offset..),
                (s.slice(..offset - x), count_to_emphasis(left_count)),
            );
        }
        // left > right, we need push new delimiter with new associated count
        (x, y) if x > y => {
            // shrink delimiter and slice, re-push it to stack
            let offset = input.offset(s);
            stack.0.push(Delimiter {
                delimiter: decrease_delimiter_count(matched.delimiter, x - y),
                slice: input.slice(offset + x - y..),
                active: true,
            });

            let offset = s.offset(o);
            return (
                s.slice(offset..),
                (s.slice(..offset - y), count_to_emphasis(left_count)),
            );
        }
        _ => unreachable!(),
    }
}

fn stack(input: &str) -> IResult<&str, EmphasisStack> {
    let mut emphasises = EmphasisStack::new();
    let mut stack = DelimiterStack::default();
    let mut i = input.clone();

    while eof::<_, Error<&str>>(i).is_err() {
        if stack.0.is_empty() {
            // stack is empty, so we must find first left flank
            if let Ok((o, t)) = left_flank(i) {
                // then push Delimiter to stack
                stack.0.push(Delimiter {
                    delimiter: t,
                    slice: o,
                    active: true,
                });
                i = o;
                continue;
            }
            // cannot find, just break, return Err finally
            break;
        } else {
            // stack is not empty, so we try to find right flank
            if let Ok((o, t)) = right_flank(i) {
                // reversely find first flank with same type
                let index = stack
                    .0
                    .iter()
                    .rev()
                    .position(|e| is_same_delimiter_type(&e.delimiter, &t));
                if let Some(index) = index {
                    // we can match right flank, return matched emphasis
                    let (r, em) = truncate_until_matched_delimiter(input, o, &mut stack, &t, index);
                    i = r;
                    emphasises.push(em);
                    continue;
                }
            }
            // not find right flank with matchable delimiter type
            // try to find next left_flank
            if let Ok((o, t)) = left_flank(i) {
                stack.0.push(Delimiter {
                    delimiter: t,
                    slice: o,
                    active: true,
                });
                i = o;
                continue;
            }
        }
        break;
    }

    if !emphasises.is_empty() {
        Ok((i, emphasises))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Eof)))
    }
}

fn emphasis(input: &str) -> IResult<&str, EmphasisStack> {
    context("emphasis", stack)(input)
}

#[test]
fn emphasis_test() {
    assert_eq!(
        emphasis("**test**"),
        Ok(("", vec![("test", EmphasisStyle::BOLD)]))
    );
    assert_eq!(
        emphasis("***test***"),
        Ok(("", vec![("test", EmphasisStyle::BOLDITALIC)]))
    );
    assert_eq!(
        emphasis("**_test_**"),
        Ok((
            "",
            vec![
                ("test", EmphasisStyle::ITALIC),
                ("_test_", EmphasisStyle::BOLD)
            ]
        ))
    );
}

pub fn parse_emphasis(input: &str) -> IResult<&str, Vec<Inline>> {
    map(emphasis, |r| {
        r.iter()
            .map(|(content, style)| {
                Inline::Emphasis(Emphasis {
                    content,
                    style: *style,
                })
            })
            .collect()
    })(input)
}
