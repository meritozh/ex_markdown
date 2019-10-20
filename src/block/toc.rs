use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::map, sequence::terminated,
    IResult,
};

use crate::token::Block;

fn toc(input: &str) -> IResult<&str, &str> {
    tag("[TOC]")(input)
}

pub fn parse_toc(input: &str) -> IResult<&str, Block> {
    map(terminated(toc, line_ending), |_| Block::TOC)(input)
}
