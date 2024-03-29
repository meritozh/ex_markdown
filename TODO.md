## Features

### Block

- [x] heading
- [x] paragraph
- [x] thematic break
- [x] blank line
- [x] front matter (except yaml parsing)
- [x] code block
- [x] latex block
- [x] blockquote (except indented blockqoute and empty blockquote)
- [x] list
- [x] task
- [x] table of content
- [x] footnote
- [x] command
- [x] container
- [x] reference

### Inline

- [x] span
- [x] text
- [x] emphasis (contains italic and bold)
- [x] strikethrough
- [x] latex
- [x] diff
- [x] subscript
- [x] superscript
- [x] image
- [x] link
- [x] ruby
- [x] mark

## TODO

1. the trailing `line_ending` in all block parsers should be optional.
2. error handling when`digit.parser` in _list.rs_.
3. ignorable indentation for some block parsers.
4. all parsers add `context`.
5. make inline parser recursive.
6. all string should be hold by `Text` token.
7. `many_till` or `terminated`
8. use `&str.offset()`
9. port `take_until_parser_match`
10. use `trim`
11. check all `take_until`, do we consume it later?
12. use pair instead of tuple
13. remove `id_tree` dependency
