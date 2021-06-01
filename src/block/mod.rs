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

use id_tree::{InsertBehavior::UnderNode, Node, NodeId};
use nom::{branch::alt, IResult};

use self::{
    blank_line::parse_blank_line, blockquote::parse_blockquote, code_block::parse_code_block,
    command::parse_command, container::parse_container, definition::parse_definition,
    footnote::parse_footnote, heading::parse_heading, import::parse_import,
    latex_block::parse_latex_block, list::parse_list, paragraph::parse_paragraph,
    thematic_break::parse_thematic_break, toc::parse_toc,
};

use super::{
    token::{Block, Token},
    Parser,
};

fn parse_block(input: &str) -> IResult<&str, Block> {
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

pub(crate) fn parse_front_matter<'a>(
    input: &'a str,
    parent: &NodeId,
    parser: &mut Parser<'a>,
) -> &'a str {
    if let Ok((i, t)) = front_matter::parse_front_matter(input) {
        let _ = parser
            .tree
            .insert(Node::new(Token::Block(t)), UnderNode(&parent));
        return i;
    }
    return input;
}

pub(crate) fn parse_first_pass<'a>(
    input: &'a str,
    parent: &NodeId,
    parser: &mut Parser<'a>,
) -> &'a str {
    let mut next = input;
    while let Ok((i, t)) = parse_block(next) {
        match t {
            Block::Definition(_) => {
                let node_id = push_token(t, parser, parent);
                parser.definitions.push(node_id);
            }
            Block::Heading(_) => {
                let node_id = push_token(t, parser, parent);
                parser.headings.push(node_id);
            }
            _ => {
                let _ = push_token(t, parser, parent);
            }
        };

        next = i;
    }
    return next;
}

fn push_token<'a>(t: Block<'a>, parser: &mut Parser<'a>, parent: &NodeId) -> NodeId {
    let node_id = parser
        .tree
        .insert(Node::new(Token::Block(t)), UnderNode(&parent))
        .unwrap();
    node_id
}
