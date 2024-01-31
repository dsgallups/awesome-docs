use crate::document::{TextDocument, TextDocumentDefinition};

use super::TextContent;

pub struct Ordered;
pub struct Unordered;

pub struct List<T> {
    children: Vec<String>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List {
            children: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl List<Unordered> {
    pub fn new(children: Vec<String>) -> Self {
        Self {
            children,
            _marker: std::marker::PhantomData,
        }
    }
}

impl List<Ordered> {
    pub fn new(children: Vec<String>) -> Self {
        Self {
            children,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> List<T> {
    pub fn new_ordered() -> Self {
        Self {
            children: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }
    pub fn new_unordered() -> Self {
        Self {
            children: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }
    pub fn add_child(mut self, child: impl Into<String>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn add_children(mut self, children: Vec<String>) -> Self {
        self.children.extend(children);
        self
    }
    pub fn ordered_list(self) -> List<Ordered> {
        List {
            children: self.children,
            _marker: std::marker::PhantomData,
        }
    }
    pub fn unordered_list(self) -> List<Unordered> {
        List {
            children: self.children,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> TextContent for List<T> {}

impl<T> From<List<T>> for TextDocumentDefinition {
    fn from(list: List<T>) -> Self {
        TextDocumentDefinition::new(String::new(), list)
    }
}
