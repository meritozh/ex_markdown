use nom::{
    bytes::complete::tag_no_case, character::complete::line_ending, combinator::map,
    error::context, sequence::terminated, IResult,
};

use crate::token::Block;

fn toc(input: &str) -> IResult<&str, &str> {
    context("toc", terminated(tag_no_case("[toc]"), line_ending))(input)
}

pub fn parse_toc(input: &str) -> IResult<&str, Block> {
    map(toc, |_| Block::TOC)(input)
}
