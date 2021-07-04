mod diff;
mod emphasis;
mod image;
mod latex;
mod link;
mod mark;
mod reference;
mod ruby;
mod span;
mod strikethrough;
mod subscript;
mod superscript;
mod text;

mod shared;

use std::collections::HashMap;

use nom::{
    branch::alt, bytes::streaming::take_till, combinator::eof, error::Error, AsChar, IResult,
};

use static_init::dynamic;

use super::token::Inline;

use self::{
    diff::parse_diff, emphasis::parse_emphasis, image::parse_image, latex::parse_latex,
    link::parse_link, mark::parse_mark, reference::parse_reference, ruby::parse_ruby,
    span::parse_span, strikethrough::parse_strikethrough, subscript::parse_subscript,
    superscript::parse_superscript, text::parse_text,
};

type Parser = fn(&str) -> IResult<&str, Inline>;

#[dynamic(lazy)]
static FAST_PARSER_MAP: HashMap<char, Parser> = {
    let mut map: HashMap<char, Parser> = HashMap::new();

    let parse_link_or_ref: Parser = |input| alt((parse_link, parse_reference))(input);
    let parse_strike_or_sub: Parser = |input| alt((parse_strikethrough, parse_subscript))(input);

    map.insert('+', parse_diff);
    map.insert('-', parse_diff);
    map.insert('`', parse_span);
    map.insert('$', parse_latex);
    map.insert('!', parse_image);
    map.insert('[', parse_link_or_ref);
    map.insert('~', parse_strike_or_sub);
    map.insert('=', parse_mark);
    map.insert('{', parse_ruby);
    map.insert('^', parse_superscript);
    map.insert('*', parse_emphasis);
    map.insert('_', parse_emphasis);

    map
};

pub(crate) fn parse_inline(input: &str) -> Vec<Inline> {
    let mut tokens = vec![];
    let mut i = input;

    while eof::<_, Error<&str>>(i).is_err() {
        // fast finding proper parser
        if let Ok((next, leading)) =
            take_till::<_, _, Error<&str>>(|c| FAST_PARSER_MAP.contains_key(&c))(i)
        {
            // get the parser
            let c = next.as_bytes()[0].as_char();
            let &p = FAST_PARSER_MAP.get(&c).unwrap();

            // if parser can handle it
            if let Ok((r, t)) = p(next) {
                // push leading as Inline::Text
                if let Ok((_, t)) = parse_text(leading) {
                    tokens.push(t);
                }
                // push established token
                tokens.push(t);
                // update i
                i = r;
            }
        } else if let Ok((r, t)) = parse_text(i) {
            tokens.push(t);
            i = r;
        }
    }

    tokens
}
