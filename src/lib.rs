mod block;
mod inline;
mod output;
mod parser;
mod token;

pub(crate) mod utils;

use parser::Parser;

use crate::output::html_writer::HtmlWriter;

pub fn parse_markdown(input: &str) -> std::io::Result<()> {
    let mut parser = Parser::default();
    parser.ext(input);

    let mut html_writer = HtmlWriter::new(Vec::new());
    html_writer.to_html(parser.tree.subtree.into_iter())?;

    let output = html_writer.to_string();
    println!("{}", output);

    Ok(())
}
