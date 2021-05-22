pub(crate) enum DelimiterType {
    // [
    OpenBracket,
    // ![
    MarkOpenBracket,
}

pub(crate) struct Delimiter<'a> {
    pub delimiter: DelimiterType,
    pub slice: &'a str,
}

#[derive(Default)]
pub(crate) struct DelimiterStack<'a>(pub Vec<Delimiter<'a>>);
