pub(crate) mod diff;
pub(crate) mod emphasis;
pub(crate) mod latex;
pub(crate) mod span;
pub(crate) mod strikethrough;
pub(crate) mod subscript;
pub(crate) mod superscript;
pub(crate) mod text;

use nom::branch::alt;

use crate::{
    inline::{
        diff::parse_diff, emphasis::parse_emphasis, latex::parse_latex, span::parse_span,
        strikethrough::parse_strikethrough, subscript::parse_subscript,
        superscript::parse_superscript, text::parse_text,
    },
    token::Inline,
};

pub fn parse_inline(input: &str) -> Vec<Inline> {
    let mut cur_input = input;
    let mut tokens: Vec<Inline> = Vec::new();
    while !cur_input.is_empty() {
        let (next_input, (token1, token2)) = alt((
            parse_span,
            parse_latex,
            parse_diff,
            parse_emphasis,
            parse_strikethrough,
            parse_subscript,
            parse_superscript,
            parse_text,
        ))(cur_input)
        .unwrap();
        vec![token1, token2].into_iter().for_each(|tk| match tk {
            Inline::Placeholder => {}
            _ => tokens.push(tk),
        });
        cur_input = next_input;
    }
    tokens
}
