#[derive(Default)]
pub(crate) struct DelimiterStack<'a>(pub Vec<&'a str>);
