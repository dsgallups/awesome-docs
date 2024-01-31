mod builder;

pub use builder::*;

use crate::content::OrderedList;

pub trait TextDocument {
    fn new(title: String, content: OrderedList) -> Self;
}
