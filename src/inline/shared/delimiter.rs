#[derive(Debug)]
pub(crate) enum DelimiterType {
    // [
    OpenBracket,
    // ![
    MarkOpenBracket,
    // *
    Asterisk(usize),
    // _
    Underline(usize),
}

#[derive(Debug)]
pub(crate) struct Delimiter<'a> {
    pub delimiter: DelimiterType,
    pub slice: &'a str,
}

#[derive(Default, Debug)]
pub(crate) struct DelimiterStack<'a>(pub(crate) Vec<Delimiter<'a>>);

pub(crate) fn is_same_delimiter_type(a: &DelimiterType, b: &DelimiterType) -> bool {
    matches!(
        (a, b),
        (&DelimiterType::Asterisk(_), &DelimiterType::Asterisk(_))
            | (&DelimiterType::Underline(_), &DelimiterType::Underline(_))
    )
}

pub(crate) fn get_delimiter_associate_count(d: &DelimiterType) -> usize {
    match *d {
        DelimiterType::Asterisk(x) => x,
        DelimiterType::Underline(x) => x,
        _ => unreachable!(),
    }
}

pub(crate) fn decrease_delimiter_count(d: DelimiterType, c: usize) -> DelimiterType {
    match d {
        DelimiterType::Asterisk(x) => DelimiterType::Asterisk(x - c),
        DelimiterType::Underline(x) => DelimiterType::Underline(x - c),
        _ => unreachable!(),
    }
}
