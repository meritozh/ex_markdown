use nom::{
    branch::alt, bytes::complete::tag, character::complete::line_ending, combinator::map,
    error::context, sequence::terminated, IResult,
};

use crate::token::Block;

fn toc(input: &str) -> IResult<&str, &str> {
    context("toc", alt((tag("[toc]"), tag("[TOC]"))))(input)
}

pub fn parse_toc(input: &str) -> IResult<&str, Block> {
    map(terminated(toc, line_ending), |_| Block::TOC)(input)
}
