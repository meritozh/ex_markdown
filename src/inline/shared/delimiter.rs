use nom::{
    bytes::complete::take_until, character::complete::char, combinator::map, sequence::tuple,
    IResult,
};

pub(crate) enum DelimiterType {
    // [
    OpenBracket,
    // ![
    MarkOpenBracket,
    // ]
    CloseBracket,
}

pub(crate) struct Delimiter<'a> {
    pub delimiter: DelimiterType,
    pub slice: &'a str,
    pub active: bool,
}

#[derive(Default)]
pub(crate) struct DelimiterStack<'a>(pub Vec<Delimiter<'a>>);

pub(crate) fn close(input: &str) -> IResult<&str, DelimiterType> {
    map(tuple((take_until("]"), char(']'))), |_| {
        DelimiterType::CloseBracket
    })(input)
}
