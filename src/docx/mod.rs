use crate::{
    content::{List, Ordered, Text},
    document::{TextDocument, TextDocumentDefinition},
};

pub struct Docx {
    title: String,
    content: TextDocumentDefinition,
}

impl TextDocument for Docx {
    fn new(title: String, content: TextDocumentDefinition) -> Self {
        Self { title, content }
    }
}
