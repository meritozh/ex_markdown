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

- [x] italic
- [ ] bold
- [ ] text
- [ ] emphasis
- [ ] delete
- [ ] sub
- [ ] sup
- [ ] ins
- [ ] dec
- [ ] mark

## TODO

1. the trailing `line_ending` in all block parsers should be optional.
2. error handling when`digit.parser` in _list.rs_.
3. ignorable indentation for some block parsers.
4. all parsers add `context`.
