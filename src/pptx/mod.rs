use crate::document::TextDocument;

pub struct Pptx {
    slides: Vec<Slide>,
}

impl TextDocument for Pptx {
    fn new(title: String, content: crate::content::OrderedList) -> Self {
        let slide = Slide { title, content };
        Self {
            slides: vec![slide],
        }
    }
}

pub struct Slide {
    title: String,
    content: crate::content::OrderedList,
}
