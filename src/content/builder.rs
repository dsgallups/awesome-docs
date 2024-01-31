use super::{List, Ordered, Text};

pub struct NoContent;

pub struct ContentBuilder<T> {
    value: Vec<String>,
    _marker: std::marker::PhantomData<T>,
}

impl Default for ContentBuilder<Text> {
    fn default() -> Self {
        ContentBuilder::new()
    }
}

impl ContentBuilder<Text> {
    pub fn new() -> Self {
        ContentBuilder {
            value: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }
    pub fn build(self) -> Text {
        Text::new(self.value.concat())
    }
}

impl<T> ContentBuilder<T> {
    pub fn add_child(mut self, child: impl Into<String>) -> Self {
        self.value.push(child.into());
        self
    }

    pub fn with_children(mut self, children: Vec<String>) -> Self {
        self.value = children;
        self
    }

    pub fn ordered_list(self) -> ContentBuilder<List<Ordered>> {
        ContentBuilder {
            value: self.value,
            _marker: std::marker::PhantomData,
        }
    }
}

impl ContentBuilder<List<Ordered>> {
    pub fn build(self) -> List<Ordered> {
        List::new_ordered().add_children(self.value)
    }
}
