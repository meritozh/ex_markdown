pub(crate) mod blank_line;
pub(crate) mod blockquote;
pub(crate) mod code_block;
pub(crate) mod front_matter;
pub(crate) mod heading;
pub(crate) mod latex_block;
pub(crate) mod list;
pub(crate) mod paragraph;
pub(crate) mod thematic_break;

use nom::branch::alt;

use crate::{
    block::{
        blank_line::parse_blank_line, blockquote::parse_blockquote, code_block::parse_code_block,
        front_matter::parse_front_matter, heading::parse_heading, latex_block::parse_latex_block,
        list::parse_list, paragraph::parse_paragraph, thematic_break::parse_thematic_break,
    },
    token::Token,
    Parser,
};

pub fn parse_block<'a>(parser: &mut Parser<'a>, input: &'a str) {
    let mut cur_input = input;
    while !cur_input.is_empty() {
        let (next_input, token) = alt((
            parse_front_matter,
            parse_blank_line,
            parse_thematic_break,
            parse_code_block,
            parse_latex_block,
            parse_list,
            parse_heading,
            // TODO: The continuation of `parse_blockquote` is special
            parse_blockquote,
            parse_paragraph,
        ))(cur_input)
        .unwrap();
        parser.push(Token::Block(token));
        cur_input = next_input;
    }
}
