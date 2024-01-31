use crate::{content::OrderedList, document::TextDocument};

pub struct Docx {
    title: String,
    content: OrderedList,
}

impl TextDocument for Docx {
    fn new(title: String, content: OrderedList) -> Self {
        Self { title, content }
    }
}
