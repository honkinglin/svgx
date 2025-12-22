use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
    Cdata(String),
    Doctype(String),
    ProcessingInstruction(String, Option<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub name: String,
    pub attributes: IndexMap<String, String>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: IndexMap::new(),
            children: Vec::new(),
        }
    }
}

pub struct Document {
    pub root: Vec<Node>, // Usually contains one root Element, but can have comments/doctype before it
}

impl Document {
    pub fn new() -> Self {
        Self { root: Vec::new() }
    }
}
