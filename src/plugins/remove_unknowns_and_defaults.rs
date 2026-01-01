use crate::plugins::Plugin;
use crate::tree::{Document, Node};
use std::collections::HashMap;

pub struct RemoveUnknownsAndDefaults {
    pub default_attrs: HashMap<&'static str, &'static str>,
}

impl Default for RemoveUnknownsAndDefaults {
    fn default() -> Self {
        let mut map = HashMap::new();
        // Common SVG defaults
        map.insert("cx", "0");
        map.insert("cy", "0");
        map.insert("x", "0");
        map.insert("y", "0");
        map.insert("r", "0");
        map.insert("rx", "0");
        map.insert("ry", "0");
        // Transformations
        map.insert("rotate", "0");
        map.insert("scale", "1");

        // Paint
        map.insert("stroke-width", "1");
        map.insert("stroke-opacity", "1");
        map.insert("fill-opacity", "1");
        map.insert("stop-opacity", "1");

        // Text
        map.insert("letter-spacing", "normal"); // usually 0? Check syntax
        map.insert("word-spacing", "normal");

        // Groups
        // opacity=1

        Self { default_attrs: map }
    }
}

impl Plugin for RemoveUnknownsAndDefaults {
    fn apply(&self, doc: &mut Document) {
        process_nodes(&mut doc.root, &self.default_attrs);
    }
}

fn process_nodes(nodes: &mut Vec<Node>, defaults: &HashMap<&str, &str>) {
    for node in nodes {
        if let Node::Element(elem) = node {
            // Remove defaults
            // We need to check if value matches default.
            // SVG defaults often depend on context or units.
            // We'll stick to safe, unitless 0 cases effectively.

            // "stroke-width" default is "1". If "1px", also "1".
            // Need smart check? For now, strict string match or simple numeric.

            elem.attributes.retain(|k, v| {
                if let Some(def) = defaults.get(k.as_str()) {
                    if v == *def {
                        return false;
                    }

                    // Simple number check: "0" == "0px" == "0pt"
                    if *def == "0" && (v == "0px" || v == "0pt" || v == "0em") {
                        return false;
                    }
                    if *def == "1" && (v == "1px") {
                        // stroke-width=1px is same as 1
                        return false;
                    }
                }
                true
            });

            // Remove unknowns?
            // e.g. xmlns:inkscape ...
            // That's Namespace cleanup.
            // Let's stick to Defaults here.

            process_nodes(&mut elem.children, defaults);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_remove_defaults() {
        let input = "<svg><rect x=\"0\" y=\"0\" width=\"100\" stroke-width=\"1\"/></svg>";
        let expected = "<svg><rect width=\"100\"/></svg>";

        let mut doc = parser::parse(input).unwrap();
        RemoveUnknownsAndDefaults::default().apply(&mut doc);
        assert_eq!(printer::print(&doc), expected);
    }
}
