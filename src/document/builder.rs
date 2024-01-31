use crate::{content::OrderedList, pdf::Pdf};

use super::TextDocument;

#[derive(Default)]
pub struct DocumentBuilder {
    title: String,
    definition: OrderedList,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_definition(mut self, definition: OrderedList) -> Self {
        self.definition = definition;
        self
    }

    pub fn build<T>(self) -> T
    where
        T: TextDocument,
    {
        T::new(self.title, self.definition)
    }
}
