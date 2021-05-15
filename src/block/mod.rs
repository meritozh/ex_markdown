pub(crate) mod blank_line;
pub(crate) mod blockquote;
pub(crate) mod code_block;
pub(crate) mod command;
pub(crate) mod container;
pub(crate) mod definition;
pub(crate) mod footnote;
pub(crate) mod front_matter;
pub(crate) mod heading;
pub(crate) mod latex_block;
pub(crate) mod list;
pub(crate) mod paragraph;
pub(crate) mod thematic_break;
pub(crate) mod toc;

use nom::branch::alt;

use crate::{
    block::{
        blank_line::parse_blank_line, blockquote::parse_blockquote, code_block::parse_code_block,
        command::parse_command, container::parse_container, definition::parse_definition,
        front_matter::parse_front_matter, heading::parse_heading, latex_block::parse_latex_block,
        list::parse_list, paragraph::parse_paragraph, thematic_break::parse_thematic_break,
        toc::parse_toc,
    },
    token::Token,
    Parser,
};

use self::footnote::parse_footnote;

pub fn parse_block<'a>(parser: &mut Parser<'a>, input: &'a str) {
    let mut cur_input = input;
    while !cur_input.is_empty() {
        let (next_input, token) = alt((
            parse_toc,
            parse_definition,
            parse_front_matter,
            parse_blank_line,
            parse_thematic_break,
            parse_container,
            parse_command,
            parse_code_block,
            parse_latex_block,
            parse_list,
            parse_heading,
            parse_footnote,
            // TODO: The continuation of `parse_blockquote` is special
            parse_blockquote,
            parse_paragraph,
        ))(cur_input)
        .unwrap();
        parser.push(Token::Block(token));
        cur_input = next_input;
    }
}
