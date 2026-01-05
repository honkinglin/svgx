use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct RemoveEmptyContainers;

impl Plugin for RemoveEmptyContainers {
    fn apply(&self, doc: &mut Document) {
        process_nodes(&mut doc.root);
    }
}

fn process_nodes(nodes: &mut Vec<Node>) {
    nodes.retain(|node| {
        if let Node::Element(elem) = node {
            // List of container tags
            let is_container = matches!(
                elem.name.as_str(),
                "defs"
                    | "g"
                    | "marker"
                    | "mask"
                    | "missing-glyph"
                    | "pattern"
                    | "switch"
                    | "symbol"
            );

            if is_container && elem.children.is_empty() {
                // If it has id, carefully considering removal?
                // svgo removeEmptyContainers removes them even if they have ID,
                // UNLESS it's "svg" (root) or inner svg.
                // But wait, pattern/mask with ID but empty is useless (renders nothing).
                // So safe to remove.
                return false;
            }
        }
        true
    });

    for node in nodes {
        if let Node::Element(elem) = node {
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
    fn test_remove_empty_g() {
        let input = "<svg><g/></svg>";
        let expected = "<svg></svg>"; // Expect <svg/> or <svg></svg> depending on printer (parser produces children vec, if empty, printer prints short if no children)
                                      // Actually printer prints <svg></svg> (with children). Wait.
                                      // My printer always uses self-closing for empty? No?
                                      // Let's check expected. Parser returns Node::Element("svg", [], []).
                                      // Printer: if children empty, print self-closing "/>".
                                      // Wait, input <svg><g/></svg>. svg has 1 child (g). g has 0 children.
                                      // removeEmptyContainers removes g. svg has 0 children.
                                      // Printer prints <svg/> (self closing).

        let mut doc = parser::parse(input).unwrap();
        RemoveEmptyContainers.apply(&mut doc);

        // However, the test output expectation depends on how parser/printer handles root.
        // If my printer logic handles root self-closing, it should be <svg/>.

        // Let's use flexible assertion or match my printer code.
        // Previous tests used <svg></svg> expectation for empty content inside?
        // Let's check `test_remove_invisible_rect` in `remove_useless_stroke_and_fill`.
        // It failed because it expected `<svg></svg>` but got `<svg/>`.
        // So my printer DOES print self-closing!

        let output = printer::print(&doc);
        assert!(output == "<svg/>" || output == "<svg></svg>");
    }

    #[test]
    fn test_remove_empty_defs() {
        // <defs/> usually removed by removeUselessDefs, but this catches it too
        let input = "<svg><defs/></svg>";
        let mut doc = parser::parse(input).unwrap();
        RemoveEmptyContainers.apply(&mut doc);
        let output = printer::print(&doc);
        assert!(output == "<svg/>" || output == "<svg></svg>");
    }

    #[test]
    fn test_keep_filled() {
        let input = "<svg><g><rect/></g></svg>";
        let mut doc = parser::parse(input).unwrap();
        RemoveEmptyContainers.apply(&mut doc);
        assert_eq!(printer::print(&doc), input);
    }
}
