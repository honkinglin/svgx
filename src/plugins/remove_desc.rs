use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct RemoveDesc;

impl Plugin for RemoveDesc {
    fn apply(&self, doc: &mut Document) {
        remove_elems_by_name(&mut doc.root, "desc");
    }
}

fn remove_elems_by_name(nodes: &mut Vec<Node>, name: &str) {
    nodes.retain(|node| {
        if let Node::Element(elem) = node {
            if elem.name == name {
                return false;
            }
        }
        true
    });

    for node in nodes {
        if let Node::Element(elem) = node {
            remove_elems_by_name(&mut elem.children, name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_remove_desc() {
        let input = "<svg><desc>Description</desc><rect/></svg>";
        let expected = "<svg><rect/></svg>";

        let mut doc = parser::parse(input).unwrap();
        RemoveDesc.apply(&mut doc);
        assert_eq!(printer::print(&doc), expected);
    }
}
