use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct RemoveUselessStrokeAndFill;

impl Plugin for RemoveUselessStrokeAndFill {
    fn apply(&self, doc: &mut Document) {
        process_nodes(&mut doc.root);
    }
}

fn process_nodes(nodes: &mut Vec<Node>) {
    nodes.retain(|node| {
        if let Node::Element(elem) = node {
            // Check for invisible elements
            // If it's a shape (rect, circle, etc or path) and has no visible stroke/fill
            // Default fill is black! So fill="none" must be explicit.

            let has_stroke = if let Some(s) = elem.attributes.get("stroke") {
                s != "none"
            } else {
                false
            };

            let has_fill = if let Some(f) = elem.attributes.get("fill") {
                f != "none"
            } else {
                true
            }; // Default is black

            // If stroke is present but stroke-width is 0, it's invisible stroke.
            let visible_stroke = if has_stroke {
                if let Some(w) = elem.attributes.get("stroke-width") {
                    w != "0" && w != "0px"
                } else {
                    true
                } // Default 1
            } else {
                false
            };

            // If no visible stroke and explicit fill="none", it's invisible.
            // But we must separate "removing attribute" from "removing element".

            // Keep element if it defines something? (defs are handled elsewhere).
            // Only remove distinct shapes.
            let is_shape = matches!(
                elem.name.as_str(),
                "rect" | "circle" | "ellipse" | "line" | "polygon" | "polyline" | "path"
            );

            if is_shape && !visible_stroke && !has_fill {
                // Element is invisible
                // check for id? If it has ID it might be referenced.
                if !elem.attributes.contains_key("id") {
                    return false;
                }
            }
        }
        true
    });

    for node in nodes {
        if let Node::Element(elem) = node {
            // Attribute cleanup
            if let Some(s) = elem.attributes.get("stroke") {
                if s == "none" {
                    elem.attributes.shift_remove("stroke");
                }
            }
            if let Some(w) = elem.attributes.get("stroke-width") {
                if w == "0" || w == "0px" {
                    elem.attributes.shift_remove("stroke"); // Remove stroke definition if width 0
                    elem.attributes.shift_remove("stroke-width");
                }
            }

            // If fill is none? We usually keep fill="none" because default is black.
            // removing fill="none" makes it black!

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
    fn test_remove_invisible_rect() {
        let input = "<svg><rect fill=\"none\" stroke=\"none\"/></svg>";
        let expected = "<svg/>";
        let mut doc = parser::parse(input).unwrap();
        RemoveUselessStrokeAndFill.apply(&mut doc);
        assert_eq!(printer::print(&doc), expected);
    }

    #[test]
    fn test_keep_visible() {
        let input = "<svg><rect fill=\"none\" stroke=\"red\"/></svg>";
        // fill none kept because default is black
        let mut doc = parser::parse(input).unwrap();
        RemoveUselessStrokeAndFill.apply(&mut doc);
        assert_eq!(printer::print(&doc), input);
    }

    #[test]
    fn test_remove_zero_width_stroke() {
        let input = "<svg><rect stroke=\"red\" stroke-width=\"0\"/></svg>";
        let expected = "<svg><rect/></svg>"; // stroke removed, default fill (black) remains implied?
                                             // Wait, input has no fill, so default is black.
                                             // Result has no attributes, so default black. Correct.

        // If input was fill="none" stroke="red" width="0"
        // stroke removed -> invisible -> element removed?

        let mut doc = parser::parse(input).unwrap();
        RemoveUselessStrokeAndFill.apply(&mut doc);
        // stroke and stroke-width removed.
        assert_eq!(printer::print(&doc), expected);
    }
}
