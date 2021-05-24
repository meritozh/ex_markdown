use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::{eof, map, not, peek},
    error::{context, Error, ErrorKind},
    multi::many1_count,
    sequence::{preceded, terminated},
    Err, IResult, Offset, Slice,
};

use crate::{token::EmphasisStyle, utils::nom_extend::take_until_parser_matches};

use super::shared::delimiter::{
    decrease_delimiter_count, get_delimiter_associate_count, is_same_delimiter_type, Delimiter,
    DelimiterStack, DelimiterType,
};

fn mark(input: &str) -> IResult<&str, char> {
    alt((char('*'), char('_')))(input)
}

fn left_flank(input: &str) -> IResult<&str, DelimiterType> {
    let ch = mark(input)?.1;

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
        1 => EmphasisStyle::Italic,
        x if x % 2 == 0 => EmphasisStyle::Bold,
        x if x % 2 == 1 => EmphasisStyle::BoldItalic,
        _ => unreachable!(),
    }
}

type EmphasisStack<'a> = Vec<(&'a str, EmphasisStyle)>;

fn stack(input: &str) -> IResult<&str, EmphasisStack> {
    let mut emphasises = EmphasisStack::new();
    let mut stack = DelimiterStack::default();
    let mut i = input.clone();

    while eof::<_, Error<&str>>(i).is_err() {
        // find left flank, push to stack
        if let Ok((o, t)) = left_flank(i) {
            stack.0.push(Delimiter {
                delimiter: t,
                slice: o,
                active: true,
            });
            i = o;
            continue;
            // find right flank, try to pop paired flank
        } else if let Ok((o, t)) = right_flank(i) {
            // reversely find first flank with same type
            let index = stack
                .0
                .iter()
                .rev()
                .position(|e| is_same_delimiter_type(&e.delimiter, &t));
            if let Some(index) = index {
                // stack pop to index
                stack.0.truncate(index + 1);
                // SAFETY: truncate guarantee stack is not empty
                let e = stack.0.pop().unwrap();

                // left and right frank count, messy logic start
                let left_count = get_delimiter_associate_count(&e.delimiter);
                let right_count = get_delimiter_associate_count(&t);

                // the result emphasis, with its style
                let em: (&str, EmphasisStyle);
                match (left_count, right_count) {
                    // left < right, so we need forward left (left - right)
                    (x, y) if x < y => {
                        let offset = i.offset(o) - (y - x);
                        em = (i.slice(..offset - x), count_to_emphasis(left_count));
                        i = i.slice(offset..);
                    }
                    // left == right, all thing is perfect
                    (x, y) if x == y => {
                        let offset = i.offset(o);
                        em = (i.slice(..offset - x), count_to_emphasis(left_count));
                        i = i.slice(offset..);
                    }
                    // left > right, we need push new delimiter with new associated count
                    (x, y) if x > y => {
                        // shrink delimiter and slice, re-push it to stack
                        let offset = input.offset(i);
                        stack.0.push(Delimiter {
                            delimiter: decrease_delimiter_count(e.delimiter, x - y),
                            slice: input.slice(offset + x - y..),
                            active: true,
                        });

                        let offset = i.offset(o);
                        em = (i.slice(..offset - y), count_to_emphasis(left_count));
                        i = i.slice(offset..);
                    }
                    _ => unreachable!(),
                };
                emphasises.push(em);
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
        Ok(("", vec![("test", EmphasisStyle::Bold)]))
    );
    assert_eq!(
        emphasis("***test***"),
        Ok(("", vec![("test", EmphasisStyle::BoldItalic)]))
    );
    assert_eq!(
        emphasis("**_test_**"),
        Ok(("", vec![("test", EmphasisStyle::BoldItalic)]))
    );
    assert_eq!(
        emphasis("*__test__*"),
        Ok(("", vec![("test", EmphasisStyle::BoldItalic)]))
    );
}
