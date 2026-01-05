use crate::plugins::Plugin;
use crate::tree::{Document, Node};
use std::collections::HashMap;

pub struct SortDefsChildren;

impl Plugin for SortDefsChildren {
    fn apply(&self, doc: &mut Document) {
        process_nodes(&mut doc.root);
    }
}

fn process_nodes(nodes: &mut Vec<Node>) {
    for node in nodes {
        if let Node::Element(elem) = node {
            if elem.name == "defs" {
                // Sort children by tag name
                elem.children.sort_by(|a, b| {
                    let name_a = get_name(a);
                    let name_b = get_name(b);
                    name_a.cmp(&name_b)
                });
            }
            process_nodes(&mut elem.children);
        }
    }
}

fn get_name(node: &Node) -> String {
    match node {
        Node::Element(e) => e.name.clone(),
        Node::Text(_) => "".to_string(), // Text usually shouldn't be valid child of defs for sorting purposes?
        Node::Comment(_) => "".to_string(),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_sort_defs() {
        let input = "<svg><defs><rect id=\"b\"/><circle id=\"a\"/></defs></svg>";
        // expectation: circle before rect (alphabetical by tag)
        let expected = "<svg><defs><circle id=\"a\"/><rect id=\"b\"/></defs></svg>";

        let mut doc = parser::parse(input).unwrap();
        SortDefsChildren.apply(&mut doc);
        assert_eq!(printer::print(&doc), expected);
    }
}
