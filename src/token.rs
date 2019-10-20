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
    BlankLine,
    ThematicBreak,
    TOC,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FrontMatter<'a> {
    pub child: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paragraph<'a> {
    // pub children: Vec<Inline<'a>>,
    pub child: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockQuote<'a> {
    // Vec here, because it allow empty line.
    pub child: &'a str,
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
    pub child: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandTag {
    Table,
    Chart,
    Youtube,
    Twitter,
    Bilibili,
    Weibo,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command<'a> {
    pub tag: CommandTag,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeBlock<'a> {
    pub lang: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LatexBlock<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RefDetail<'a> {
    pub symvbol: RefSymbol<'a>,
    pub children: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Inline<'a> {
    // Raw text without any style, which mean it's the latest node.
    Text(Text<'a>),
    Link(Link<'a>),
    Bold(Bold<'a>),
    Emphasis(Emphasis<'a>),
    Italic(Italic<'a>),
    Highlight(Highlight<'a>),
    Deletion(Deletion<'a>),
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
pub struct Text<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Link<'a> {
    // `title` can be empty.
    pub title: Option<&'a str>,
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bold<'a> {
    // TODO: make it can combine with Emphasis.
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Emphasis<'a> {
    // TODO: make it can combine with Emphasis.
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Italic<'a> {
    // TODO: make it can combine with Emphasis.
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Highlight<'a> {
    pub children: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Deletion<'a> {
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
    pub children: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Image<'a> {
    // `title` can be empty.
    pub title: Option<&'a str>,
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ruby<'a> {
    pub annotation: &'a str,
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
