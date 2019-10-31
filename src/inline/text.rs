use nom::IResult;

use crate::token::{Inline, Text};

pub fn text(input: &str) -> Text {
    Text { content: input }
}

pub fn parse_text(input: &str) -> IResult<&str, (Inline, Inline)> {
    Ok(("", (Inline::Placeholder, Inline::Text(text(input)))))
}
