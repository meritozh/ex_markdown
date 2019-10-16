use crate::token::Text;

pub fn parse_text(input: &str) -> Text {
    Text { content: input }
}

#[test]
fn parse_text_test() {
    assert_eq!(
        parse_text("hello, world"),
        Text {
            content: "hello, world"
        }
    );
}
