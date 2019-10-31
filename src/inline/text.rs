use crate::token::Text;

pub fn text(input: &str) -> Text {
    Text { content: input }
}
