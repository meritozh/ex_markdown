mod blank_line;
mod blockquote;
mod code_block;
mod command;
mod container;
mod definition;
mod footnote;
mod front_matter;
mod heading;
mod import;
mod latex_block;
mod list;
mod paragraph;
mod thematic_break;
mod toc;

use nom::{branch::alt, IResult};

use super::token::Block;

use self::{
    blank_line::parse_blank_line, blockquote::parse_blockquote, code_block::parse_code_block,
    command::parse_command, container::parse_container, definition::parse_definition,
    footnote::parse_footnote, heading::parse_heading, import::parse_import,
    latex_block::parse_latex_block, list::parse_list, paragraph::parse_paragraph,
    thematic_break::parse_thematic_break, toc::parse_toc,
};

#[inline]
pub(crate) fn parse_block(input: &str) -> IResult<&str, Block> {
    alt((
        parse_thematic_break,
        parse_heading,
        parse_toc,
        parse_code_block,
        parse_latex_block,
        parse_import,
        parse_command,
        parse_footnote,
        parse_blockquote,
        parse_container,
        parse_definition,
        parse_list,
        parse_blank_line,
        parse_paragraph,
    ))(input)
}

#[inline]
pub(crate) fn parse_front_matter(input: &str) -> IResult<&str, Block> {
    front_matter::parse_front_matter(input)
}
