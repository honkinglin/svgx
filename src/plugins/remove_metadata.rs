use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct RemoveMetadata;

impl Plugin for RemoveMetadata {
    fn apply(&self, doc: &mut Document) {
        remove_metadata_from_nodes(&mut doc.root);
    }
}

fn remove_metadata_from_nodes(nodes: &mut Vec<Node>) {
    nodes.retain(|node| {
        if let Node::Element(elem) = node {
            if elem.name == "metadata" {
                return false;
            }
        }
        true
    });

    for node in nodes {
        if let Node::Element(elem) = node {
            remove_metadata_from_nodes(&mut elem.children);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_remove_metadata() {
        let input = "<svg><metadata>Some info</metadata><g><metadata>Inner info</metadata><rect/></g></svg>";
        let expected = "<svg><g><rect/></g></svg>";

        let mut doc = parser::parse(input).unwrap();
        RemoveMetadata.apply(&mut doc);
        let output = printer::print(&doc);

        assert_eq!(output, expected);
    }
}
