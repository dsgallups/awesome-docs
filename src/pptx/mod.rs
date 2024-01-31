use crate::document::{TextDocument, TextDocumentDefinition};

pub struct Pptx {
    slides: Vec<Slide>,
}

impl TextDocument for Pptx {
    fn new(title: String, content: TextDocumentDefinition) -> Self {
        let slide = Slide { title, content };
        Self {
            slides: vec![slide],
        }
    }
}

pub struct Slide {
    title: String,
    content: TextDocumentDefinition,
}
