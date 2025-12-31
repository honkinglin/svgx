use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct ConvertStyleToAttrs;

impl Plugin for ConvertStyleToAttrs {
    fn apply(&self, doc: &mut Document) {
        process_style(&mut doc.root);
    }
}

fn process_style(nodes: &mut Vec<Node>) {
    for node in nodes {
        if let Node::Element(elem) = node {
            if let Some(style_val) = elem.attributes.shift_remove("style") {
                // Parse "key: value; key2: value2"
                let props = parse_style(&style_val);
                for (k, v) in props {
                    // Only overwrite if not present? Or always?
                    // Style takes precedence over attributes. So we should overwrite.
                    elem.attributes.insert(k, v);
                }
                // style is removed (shifted out)
            }
            process_style(&mut elem.children);
        }
    }
}

fn parse_style(s: &str) -> Vec<(String, String)> {
    let mut props = Vec::new();
    // Simple split by ; then :
    // Does not handle quoted strings containing ; or :.
    // SVG style usually simple?

    for pair in s.split(';') {
        let pair = pair.trim();
        if pair.is_empty() {
            continue;
        }

        if let Some((k, v)) = pair.split_once(':') {
            let key = k.trim().to_string();
            let value = v.trim().to_string();
            if !key.is_empty() && !value.is_empty() {
                props.push((key, value));
            }
        }
    }
    props
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_convert_style() {
        let input = "<svg><rect style=\"fill: red; stroke: blue\" width=\"10\"/></svg>";
        let _expected = "<svg><rect width=\"10\" fill=\"red\" stroke=\"blue\"/></svg>";
        // Attribute order is not guaranteed by HashMap but printer usually deterministic if standard map used?
        // Wait, element uses IndexMap?
        // My struct Element has `pub attributes: IndexMap<String, String>`.
        // So order is preserved (insert appends).
        // input parsers attributes: width.
        // plugin removes style, inserts fill, then stroke.
        // Result: width, fill, stroke.

        let mut doc = parser::parse(input).unwrap();
        ConvertStyleToAttrs.apply(&mut doc);
        let out = printer::print(&doc);

        assert!(out.contains("fill=\"red\""));
        assert!(out.contains("stroke=\"blue\""));
    }
}
