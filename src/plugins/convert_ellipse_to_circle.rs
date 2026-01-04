use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct ConvertEllipseToCircle;

impl Plugin for ConvertEllipseToCircle {
    fn apply(&self, doc: &mut Document) {
        process_nodes(&mut doc.root);
    }
}

fn process_nodes(nodes: &mut Vec<Node>) {
    for node in nodes {
        if let Node::Element(elem) = node {
            if elem.name == "ellipse" {
                let rx = elem.attributes.get("rx").cloned();
                let ry = elem.attributes.get("ry").cloned();

                if let (Some(rx_val), Some(ry_val)) = (rx, ry) {
                    if rx_val == ry_val {
                        elem.name = "circle".to_string();
                        elem.attributes.shift_remove("rx");
                        elem.attributes.shift_remove("ry");
                        elem.attributes.insert("r".to_string(), rx_val);
                    }
                }
            }
            process_nodes(&mut elem.children);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_convert_ellipse() {
        let input = "<svg><ellipse cx=\"10\" cy=\"10\" rx=\"5\" ry=\"5\"/></svg>";
        let expected = "<svg><circle cx=\"10\" cy=\"10\" r=\"5\"/></svg>";

        let mut doc = parser::parse(input).unwrap();
        ConvertEllipseToCircle.apply(&mut doc);
        assert_eq!(printer::print(&doc), expected);
    }

    #[test]
    fn test_keep_ellipse() {
        let input = "<svg><ellipse rx=\"5\" ry=\"10\"/></svg>";
        let mut doc = parser::parse(input).unwrap();
        ConvertEllipseToCircle.apply(&mut doc);
        assert_eq!(printer::print(&doc), input);
    }
}
