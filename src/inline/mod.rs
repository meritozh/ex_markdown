pub(crate) mod emphasis;
pub(crate) mod span;
pub(crate) mod text;

use nom::branch::alt;

use crate::{
    inline::{emphasis::parse_emphasis, span::parse_span, text::text},
    token::Inline,
};

pub fn parse_inline<'a>(input: &'a str) -> Vec<Inline> {
    let mut cur_input = input;
    let mut tokens: Vec<Inline> = Vec::new();
    // while !cur_input.is_empty() {
    //     let (next_input, token) = alt((parse_span, parse_emphasis, parse_text))(cur_input).unwrap();
    //     tokens.push(token);
    //     cur_input = next_input;
    // }
    tokens
}
