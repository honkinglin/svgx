use crate::plugins::Plugin;
use crate::tree::{Document, Element, Node};
use regex::Regex;
use std::sync::OnceLock;

pub struct CleanupAttrs;

impl Plugin for CleanupAttrs {
    fn apply(&self, doc: &mut Document) {
        cleanup_attrs_in_nodes(&mut doc.root);
    }
}

fn cleanup_attrs_in_nodes(nodes: &mut Vec<Node>) {
    for node in nodes {
        if let Node::Element(elem) = node {
            cleanup_element_attrs(elem);
            cleanup_attrs_in_nodes(&mut elem.children);
        }
    }
}

fn cleanup_element_attrs(elem: &mut Element) {
    for (_, value) in elem.attributes.iter_mut() {
        // 1. Replace newlines/tabs with spaces
        let mut new_value = value.replace(['\n', '\r', '\t'], " ");

        // 2. Collapse multiple spaces (using simple heuristic or regex)
        // Using a regex for simplicity: \s+ -> " "
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"\s+").unwrap());
        new_value = re.replace_all(&new_value, " ").to_string();

        // 3. Trim
        new_value = new_value.trim().to_string();

        *value = new_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_cleanup_attrs() {
        let input = "<svg class=\"  foo \n bar  \t baz \"></svg>";
        let expected = "<svg class=\"foo bar baz\"/>";

        let mut doc = parser::parse(input).unwrap();
        CleanupAttrs.apply(&mut doc);
        let output = printer::print(&doc);

        assert_eq!(output, expected);
    }
}
