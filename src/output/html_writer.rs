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
                    self.write_with(b"<div>", frontmatter.content.as_bytes(), b"</div>")?;
                }
                Block::Paragraph(paragraph) => {
                    self.write_nest(b"<p>", paragraph.subtree.into_iter(), b"</p>")?;
                }
                Block::BlockQuote(blockquote) => {
                    self.write_nest(
                        b"<blockquote>",
                        blockquote.subtree.into_iter(),
                        b"</blockquote>",
                    )?;
                }
                Block::List(list) => {
                    self.write_nest(b"<li>", list.subtree.into_iter(), b"</li>")?;
                }
                Block::Heading(heading) => {
                    self.writer.write(b"<h1>")?;
                    self.inline_to_html(heading.subtree.into_iter())?;
                    self.writer.write(b"</h1>")?;
                }
                Block::Import(_) => todo!(),
                Block::Command(_) => {
                    // TODO(gaowanqiu)
                }
                Block::CodeBlock(codeblock) => {
                    self.writer.write(b"<pre><code>")?;
                    self.writer.write(codeblock.content.as_bytes())?;
                    self.writer.write(b"</code></pre>")?;
                }
                Block::LatexBlock(_) => {
                    // TODO(gaowanqiu)
                }
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
                    self.writer.write(b"<br>")?;
                }
                Block::ThematicBreak => {
                    self.writer.write(b"<hr>")?;
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
                        self.writer.write(b"<strong>")?;
                    }
                    if emphasis.style.contains(EmphasisStyle::ITALIC) {
                        self.writer.write(b"<em>")?;
                    }
                    self.inline_to_html(emphasis.subtree.into_iter())?;
                    if emphasis.style.contains(EmphasisStyle::ITALIC) {
                        self.writer.write(b"</em>")?;
                    }
                    if emphasis.style.contains(EmphasisStyle::BOLD) {
                        self.writer.write(b"</strong>")?;
                    }
                }
                Inline::Mark(mark) => {
                    self.writer.write(b"<mark>")?;
                    self.inline_to_html(mark.subtree.into_iter())?;
                    self.writer.write(b"</mark>")?;
                }
                Inline::Strikethrough(strikethrough) => {
                    self.writer.write(b"<del>")?;
                    self.inline_to_html(strikethrough.subtree.into_iter())?;
                    self.writer.write(b"</del>")?;
                }
                Inline::Diff(_) => {}
                Inline::Image(_) => todo!(),
                Inline::Ruby(_) => todo!(),
                Inline::Span(span) => {
                    self.writer.write(b"<span>")?;
                    self.writer.write(span.content.as_bytes())?;
                    self.writer.write(b"</span>")?;
                }
                Inline::Reference(_) => todo!(),
                Inline::Subscript(subscript) => {
                    self.writer.write(b"<sub>")?;
                    self.writer.write(subscript.content.as_bytes())?;
                    self.writer.write(b"</sub>")?;
                }
                Inline::Superscript(superscript) => {
                    self.writer.write(b"<sup>")?;
                    self.writer.write(superscript.content.as_bytes())?;
                    self.writer.write(b"</sup>")?;
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
        self.inline_to_html(nest)?;
        self.write(close)?;

        Ok(())
    }

    fn write_with(&mut self, open: &[u8], content: &[u8], close: &[u8]) -> std::io::Result<()> {
        self.write(open)?;
        self.write(content)?;
        self.write(close)?;
        Ok(())
    }
}

impl ToString for HtmlWriter<Vec<u8>> {
    fn to_string(&self) -> String {
        String::from_utf8(self.writer.get_ref().to_owned()).unwrap()
    }
}
