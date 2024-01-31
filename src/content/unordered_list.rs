#[derive(Default)]
pub struct OrderedList {
    children: Vec<String>,
}

impl OrderedList {
    pub fn new(children: Vec<String>) -> Self {
        Self { children }
    }
}
