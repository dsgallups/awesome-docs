use crate::document::{TextDocument, TextDocumentDefinition};

pub struct Pdf {
    title: String,
    definition: TextDocumentDefinition,
}

impl TextDocument for Pdf {
    fn new(title: String, definition: TextDocumentDefinition) -> Self {
        Self { title, definition }
    }
}
