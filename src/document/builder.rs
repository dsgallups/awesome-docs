use crate::{
    content::{List, Ordered, TextContent},
    pdf::Pdf,
};

use super::{TextDocument, TextDocumentDefinition};

pub struct DocumentBuilder {
    title: String,
    definition: TextDocumentDefinition,
}

impl Default for DocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            definition: TextDocumentDefinition::new(String::new(), List::<Ordered>::default()),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_definition(mut self, definition: impl Into<TextDocumentDefinition>) -> Self {
        self.definition = definition.into();
        self
    }

    pub fn build<T>(self) -> T
    where
        T: TextDocument,
    {
        T::new(self.title, self.definition)
    }
}
