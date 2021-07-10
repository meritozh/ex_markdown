use std::io::{BufWriter, Write};

use crate::token::{Block, EmphasisStyle, Inline};

pub struct HtmlWriter<W: Write> {
    writer: BufWriter<W>,
}

impl<W> HtmlWriter<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        HtmlWriter {
            writer: BufWriter::new(writer),
        }
    }

    pub fn to_html<'a, I>(&mut self, iterator: I) -> std::io::Result<()>
    where
        I: Iterator<Item = Block<'a>>,
    {
        for block in iterator {
            match block {
                Block::FrontMatter(frontmatter) => {
                    self.write_with(b"<div>", frontmatter.content.as_bytes(), b"</div>")?
                }
                Block::Paragraph(paragraph) => {
                    self.write_nest(b"<p>", paragraph.subtree.into_iter(), b"</p>")?
                }
                Block::BlockQuote(blockquote) => self.write_nest(
                    b"<blockquote>",
                    blockquote.subtree.into_iter(),
                    b"</blockquote>",
                )?,
                Block::List(list) => {
                    self.write_nest(b"<li>", list.subtree.into_iter(), b"</li>")?
                }
                Block::Heading(heading) => match heading.level {
                    x @ 1..=6 => {
                        let o = format!("<h{}>", x);
                        let c = format!("</h{}", x);
                        self.write_nest(o.as_bytes(), heading.subtree.into_iter(), c.as_bytes())?
                    }
                    _ => unreachable!(),
                },
                Block::Import(_) => todo!(),
                Block::Command(_) => {
                    // TODO(gaowanqiu)
                }
                Block::CodeBlock(codeblock) => self.write_with(
                    b"<pre><code>",
                    codeblock.content.as_bytes(),
                    b"</code></pre>",
                )?,
                Block::LatexBlock(latex_block) => self.write_with(
                    b"<div class='latex block'>",
                    latex_block.content.as_bytes(),
                    b"</div>",
                )?,
                Block::Definition(_) => {
                    // TODO(gaowanqiu)
                }
                Block::Footnote(_) => {
                    // TODO(gaowanqiu)
                }
                Block::Container(_) => {
                    // TODO(gaowanqiu)
                }
                Block::BlankLine => {
                    self.write(b"<br>")?;
                    self.write(b"\n")?;
                }
                Block::ThematicBreak => {
                    self.write(b"<hr>")?;
                    self.write(b"\n")?;
                }
                Block::TableOfContent => {
                    // TODO(gaowanqiu)
                }
            }
        }
        self.writer.flush()?;
        Ok(())
    }

    fn inline_to_html<'a, I>(&mut self, iterator: I) -> std::io::Result<()>
    where
        I: Iterator<Item = Inline<'a>>,
    {
        for inline in iterator {
            match inline {
                Inline::Text(text) => {
                    self.writer.write(text.content.as_bytes())?;
                }
                Inline::Link(_) => todo!(),
                Inline::Emphasis(emphasis) => {
                    if emphasis.style.contains(EmphasisStyle::BOLD) {
                        self.write(b"<strong>")?;
                    }
                    if emphasis.style.contains(EmphasisStyle::ITALIC) {
                        self.write(b"<em>")?;
                    }
                    self.inline_to_html(emphasis.subtree.into_iter())?;
                    if emphasis.style.contains(EmphasisStyle::ITALIC) {
                        self.write(b"</em>")?;
                    }
                    if emphasis.style.contains(EmphasisStyle::BOLD) {
                        self.write(b"</strong>")?;
                    }
                }
                Inline::Mark(mark) => {
                    self.write_nest(b"<mark>", mark.subtree.into_iter(), b"</mark>")?
                }
                Inline::Strikethrough(strikethrough) => {
                    self.write_nest(b"<del>", strikethrough.subtree.into_iter(), b"</del>")?
                }
                Inline::Diff(_) => {}
                Inline::Image(_) => todo!(),
                Inline::Ruby(_) => todo!(),
                Inline::Span(span) => {
                    self.write_with(b"<span>", span.content.as_bytes(), b"</span>")?
                }
                Inline::Reference(_) => todo!(),
                Inline::Subscript(subscript) => {
                    self.write_with(b"<sub>", subscript.content.as_bytes(), b"</sub>")?
                }
                Inline::Superscript(superscript) => {
                    self.write_with(b"<sup>", superscript.content.as_bytes(), b"</sup>")?
                }
                Inline::Latex(_) => todo!(),
            }
        }
        Ok(())
    }

    fn write(&mut self, content: &[u8]) -> std::io::Result<()> {
        self.writer.write(content)?;
        Ok(())
    }

    fn write_nest<'a, I>(&mut self, open: &[u8], nest: I, close: &[u8]) -> std::io::Result<()>
    where
        I: Iterator<Item = Inline<'a>>,
    {
        self.write(open)?;
        self.writer.write(b"\n")?;
        self.inline_to_html(nest)?;
        self.write(close)?;
        self.writer.write(b"\n")?;

        Ok(())
    }

    fn write_with(&mut self, open: &[u8], content: &[u8], close: &[u8]) -> std::io::Result<()> {
        self.write(open)?;
        self.writer.write(b"\n")?;
        self.write(content)?;
        self.write(close)?;
        self.writer.write(b"\n")?;
        Ok(())
    }
}

impl ToString for HtmlWriter<Vec<u8>> {
    fn to_string(&self) -> String {
        String::from_utf8(self.writer.get_ref().to_owned()).unwrap()
    }
}
