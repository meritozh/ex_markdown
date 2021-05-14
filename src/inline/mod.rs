pub(crate) mod diff;
pub(crate) mod emphasis_link;
pub(crate) mod latex;
pub(crate) mod mark;
pub(crate) mod reference;
pub(crate) mod ruby;
pub(crate) mod span;
pub(crate) mod strikethrough;
pub(crate) mod subscript;
pub(crate) mod superscript;
pub(crate) mod text;

use crate::token::Inline;

pub fn parse_inline(input: &str) -> Vec<Inline> {
    let mut cur_input = input;
    let mut tokens: Vec<Inline> = Vec::new();

    tokens
}
