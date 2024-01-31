mod builder;

pub use builder::*;

mod definition;
pub use definition::*;

pub trait TextDocument {
    fn new(title: String, definition: TextDocumentDefinition) -> Self;
}
