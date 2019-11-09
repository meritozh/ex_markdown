pub(crate) mod emphasis;
pub(crate) mod span;
pub(crate) mod strikethrough;
pub(crate) mod text;

use nom::branch::alt;

use crate::{
    inline::{
        emphasis::parse_emphasis, span::parse_span, strikethrough::parse_strikethrough,
        text::parse_text,
    },
    token::Inline,
};

pub fn parse_inline(input: &str) -> Vec<Inline> {
    let mut cur_input = input;
    let mut tokens: Vec<Inline> = Vec::new();
    while !cur_input.is_empty() {
        let (next_input, (token1, token2)) =
            alt((parse_span, parse_emphasis, parse_strikethrough, parse_text))(cur_input).unwrap();
        vec![token1, token2].into_iter().for_each(|tk| match tk {
            Inline::Placeholder => {}
            _ => tokens.push(tk),
        });
        cur_input = next_input;
    }
    tokens
}
