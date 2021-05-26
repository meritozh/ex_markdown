pub(crate) mod blank_line;
pub(crate) mod blockquote;
pub(crate) mod code_block;
pub(crate) mod command;
pub(crate) mod container;
pub(crate) mod definition;
pub(crate) mod footnote;
pub(crate) mod front_matter;
pub(crate) mod heading;
pub(crate) mod import;
pub(crate) mod latex_block;
pub(crate) mod list;
pub(crate) mod paragraph;
pub(crate) mod thematic_break;
pub(crate) mod toc;

use nom::{branch::alt, IResult};

use crate::{
    block::{
        blank_line::parse_blank_line, blockquote::parse_blockquote, code_block::parse_code_block,
        container::parse_container, definition::parse_definition, heading::parse_heading,
        import::parse_import, latex_block::parse_latex_block, list::parse_list,
        paragraph::parse_paragraph, thematic_break::parse_thematic_break, toc::parse_toc,
    },
    token::Block,
};

pub(crate) use self::front_matter::parse_front_matter;
use self::{command::parse_command, footnote::parse_footnote};

pub fn parse_first_pass(input: &str) -> IResult<&str, Block> {
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
