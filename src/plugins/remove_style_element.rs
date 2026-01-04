use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct RemoveStyleElement;

impl Plugin for RemoveStyleElement {
    fn apply(&self, doc: &mut Document) {
        remove_style_recursive(&mut doc.root);
    }
}

fn remove_style_recursive(nodes: &mut Vec<Node>) {
    nodes.retain(|node| {
        if let Node::Element(elem) = node {
            if elem.name == "style" {
                return false;
            }
        }
        true
    });

    for node in nodes {
        if let Node::Element(elem) = node {
            remove_style_recursive(&mut elem.children);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_remove_style() {
        let input = "<svg><style>.cls { fill: red; }</style><rect/></svg>";
        let expected = "<svg><rect/></svg>";

        let mut doc = parser::parse(input).unwrap();
        RemoveStyleElement.apply(&mut doc);
        assert_eq!(printer::print(&doc), expected);
    }
}
