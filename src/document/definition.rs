use crate::content::TextContent;

pub struct TextDocumentDefinition<T> {
    title: String,
    content: Box<dyn TextContent>,
}

impl<T: TextContent> TextDocumentDefinition<T> {
    pub fn new(title: String, content: T) -> Self {
        Self { title, content }
    }
}
