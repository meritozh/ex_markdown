use std::usize;

use bitflags::bitflags;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Default)]
pub struct Document<'a> {
    pub subtree: Vec<Block<'a>>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum Block<'a> {
    FrontMatter(FrontMatter<'a>),
    Paragraph(Paragraph<'a>),
    BlockQuote(BlockQuote<'a>),
    List(List<'a>),
    Heading(Heading<'a>),
    Import(Import<'a>),
    Command(Command<'a>),
    CodeBlock(CodeBlock<'a>),
    LatexBlock(LatexBlock<'a>),
    Definition(Definition<'a>),
    Footnote(Footnote<'a>),
    Container(Container<'a>),
    BlankLine,
    ThematicBreak,
    TableOfContent,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Container<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FrontMatter<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct Paragraph<'a> {
    pub content: &'a str,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct BlockQuote<'a> {
    pub level: usize,
    pub content: &'a str,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct Footnote<'a> {
    pub label: &'a str,
    pub content: &'a str,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum ListStyle {
    Number(u8),
    Bullet,
    Task(bool),
}

impl Default for ListStyle {
    fn default() -> Self {
        Self::Bullet
    }
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct List<'a> {
    pub style: ListStyle,
    pub level: usize,
    pub content: &'a str,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct Heading<'a> {
    pub level: usize,
    pub content: &'a str,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Import<'a> {
    pub path: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Command<'a> {
    pub tag: &'a str,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct CodeBlock<'a> {
    pub attributes: Vec<&'a str>,
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct LatexBlock<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Definition<'a> {
    pub label: &'a str,
    pub url: &'a str,
    pub title: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum Inline<'a> {
    Text(Text<'a>),
    Link(Link<'a>),
    Emphasis(Emphasis<'a>),
    Mark(Mark<'a>),
    Strikethrough(Strikethrough<'a>),
    Diff(Diff<'a>),
    Image(Image<'a>),
    Ruby(Ruby<'a>),
    Span(Span<'a>),
    Reference(Reference<'a>),
    Subscript(Subscript<'a>),
    Superscript(Superscript<'a>),
    Latex(Latex<'a>),
}

bitflags! {
    #[derive(Default, Serialize)]
    pub struct EmphasisStyle : u8 {
        const BOLD = 0b001;
        const ITALIC = 0b010;
        const BOLDITALIC = Self::BOLD.bits | Self::ITALIC.bits;
    }
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct Emphasis<'a> {
    pub content: &'a str,
    pub style: EmphasisStyle,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Text<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Link<'a> {
    pub label: &'a str,
    pub url: &'a str,
    pub title: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct Mark<'a> {
    pub content: &'a str,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct Strikethrough<'a> {
    pub content: &'a str,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum DiffStyle {
    Plus,
    Minus,
}

impl Default for DiffStyle {
    fn default() -> Self {
        Self::Minus
    }
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct Diff<'a> {
    pub style: DiffStyle,
    pub content: &'a str,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Image<'a> {
    pub label: &'a str,
    pub url: &'a str,
    pub title: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct Ruby<'a> {
    pub annotation: &'a str,
    pub content: &'a str,
    pub subtree: Vec<Inline<'a>>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Span<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Reference<'a> {
    pub label: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Subscript<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Superscript<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Latex<'a> {
    pub content: &'a str,
}
