pub(crate) mod blank_line;
pub(crate) mod front_matter;
pub(crate) mod heading;
pub(crate) mod paragraph;

use nom::branch::alt;

use crate::{
    block::{blank_line::parse_blank_line, heading::parse_heading, paragraph::parse_paragraph},
    token::Token,
    Parser,
};

pub fn parse_block<'a>(parser: &mut Parser<'a>, input: &'a str) {
    let mut cur_input = input;
    while !cur_input.is_empty() {
        let (next_input, token) =
            alt((parse_blank_line, parse_heading, parse_paragraph))(cur_input).unwrap();
        parser.push(Token::Block(token));
        cur_input = next_input;
    }
}
