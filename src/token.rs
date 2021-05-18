use std::usize;

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Block(Block<'a>),
    Inline(Inline<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Block<'a> {
    FrontMatter(FrontMatter<'a>),
    Paragraph(Paragraph<'a>),
    BlockQuote(BlockQuote<'a>),
    List(List<'a>),
    Heading(Heading<'a>),
    Command(Command<'a>),
    CodeBlock(CodeBlock<'a>),
    LatexBlock(LatexBlock<'a>),
    Definition(Definition<'a>),
    Footnote(Footnote<'a>),
    Container(Container<'a>),
    BlankLine,
    ThematicBreak,
    TOC,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Container<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FrontMatter<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paragraph<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockQuote<'a> {
    pub level: usize,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListStyle {
    Number(u32),
    Bullet,
    Task(bool),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Footnote<'a> {
    pub label: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct List<'a> {
    pub style: ListStyle,
    pub level: usize,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Heading<'a> {
    pub level: usize,
    pub content: &'a str,
}

/// TODO: is Command duplicated with Container?
#[derive(Debug, PartialEq, Eq)]
pub struct Command<'a> {
    pub tag: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeBlock<'a> {
    pub attributes: Vec<&'a str>,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LatexBlock<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Definition<'a> {
    pub label: &'a str,
    pub url: &'a str,
    pub title: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Inline<'a> {
    Text(Text<'a>),
    Link(Link<'a>),
    Emphasis(Emphasis<'a>),
    Mark(Mark<'a>),
    Strikethrough(Strikethrough<'a>),
    Underline(Underline<'a>),
    Diff(Diff<'a>),
    Image(Image<'a>),
    Ruby(Ruby<'a>),
    Span(Span<'a>),
    Reference(Reference<'a>),
    Subscript(Subscript<'a>),
    Superscript(Superscript<'a>),
    Latex(Latex<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum EmphasisStyle {
    Bold,
    Italic,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Emphasis<'a> {
    pub content: &'a str,
    pub style: EmphasisStyle,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Text<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Link<'a> {
    pub label: &'a str,
    pub url: &'a str,
    pub title: Option<&'a str>,
}

// TODO: consider support custom color?
#[derive(Debug, PartialEq, Eq)]
pub struct Mark<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Strikethrough<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Underline<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DiffStyle {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Diff<'a> {
    pub style: DiffStyle,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Image<'a> {
    pub label: &'a str,
    pub url: &'a str,
    pub title: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ruby<'a> {
    pub annotation: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Span<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Reference<'a> {
    pub label: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Subscript<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Superscript<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Latex<'a> {
    pub content: &'a str,
}
