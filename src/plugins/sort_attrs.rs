use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct SortAttrs;

impl Plugin for SortAttrs {
    fn apply(&self, doc: &mut Document) {
        sort_attrs_in_nodes(&mut doc.root);
    }
}

fn sort_attrs_in_nodes(nodes: &mut Vec<Node>) {
    for node in nodes {
        if let Node::Element(elem) = node {
            // Sort attributes by key
            elem.attributes.sort_keys();
            sort_attrs_in_nodes(&mut elem.children);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_sort_attrs() {
        let input = "<svg><rect width=\"10\" x=\"0\" y=\"0\" height=\"10\"/></svg>";
        // Sorted: height, width, x, y
        let expected = "<svg><rect height=\"10\" width=\"10\" x=\"0\" y=\"0\"/></svg>";

        let mut doc = parser::parse(input).unwrap();
        SortAttrs.apply(&mut doc);
        let out = printer::print(&doc);
        assert_eq!(out, expected);
    }
}
