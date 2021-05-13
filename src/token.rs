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
    RefDetail(RefDetail<'a>),
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
    pub child: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paragraph<'a> {
    pub children: Vec<Inline<'a>>,
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
    pub tag: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct List<'a> {
    pub style: ListStyle,
    pub indentation: usize,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Heading<'a> {
    pub level: usize,
    // pub child: Inline<'a>,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command<'a> {
    pub tag: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeBlock<'a> {
    pub property: Vec<&'a str>,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LatexBlock<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RefDetail<'a> {
    pub reference: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Inline<'a> {
    // Raw text without any style, which mean it's the latest node.
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
    BoldItalic,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Emphasis<'a> {
    pub leading: Option<&'a str>,
    pub content: &'a str,
    pub style: EmphasisStyle,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Text<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Link<'a> {
    pub title: Text<'a>,
    pub uri: Text<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Mark<'a> {
    pub children: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Strikethrough<'a> {
    pub children: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Underline<'a> {
    pub children: Vec<Inline<'a>>,
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
    pub title: Text<'a>,
    pub uri: Text<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ruby<'a> {
    pub annotation: Text<'a>,
    pub children: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Span<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RefSymbol<'a> {
    Number(i32),
    Tag(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Reference<'a> {
    pub symbol: RefSymbol<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Subscript<'a> {
    pub child: Text<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Superscript<'a> {
    pub child: Text<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Latex<'a> {
    pub child: Text<'a>,
}
