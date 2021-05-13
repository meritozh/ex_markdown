use nom::IResult;

use crate::token::{Inline, Text};

enum BracketType {
    Round,
    Curly,
    Square,
}

enum DelimiterType {
    // {[(
    LeftBracket(BracketType),
    // )]}
    RightBracket(BracketType),
    // _
    Underscore,
    // *
    Asterisk,
}

struct Delimiter {
    r#type: DelimiterType,
}

struct DelimiterStack<'a>(Vec<&'a str>);

pub fn inliner(input: &str) -> IResult<&str, Inline> {
    Ok(("", Inline::Text(Text { content: input })))
}
