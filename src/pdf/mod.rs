use crate::{content::OrderedList, document::TextDocument};

pub struct Pdf {
    title: String,
    content: OrderedList,
}

impl TextDocument for Pdf {
    fn new(title: String, content: OrderedList) -> Self {
        Self { title, content }
    }
}
