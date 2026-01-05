use crate::plugins::Plugin;
use crate::tree::{Document, Node};
use std::collections::HashSet;

pub struct RemoveUnusedNS;

impl Plugin for RemoveUnusedNS {
    fn apply(&self, doc: &mut Document) {
        // 1. Collect used namespaces
        let mut used_prefixes = HashSet::new();
        collect_used_prefixes(&doc.root, &mut used_prefixes);

        // 2. Filter root attributes
        process_root(&mut doc.root, &used_prefixes);
    }
}

fn collect_used_prefixes(nodes: &[Node], used: &mut HashSet<String>) {
    for node in nodes {
        if let Node::Element(elem) = node {
            // Check element name (e.g. "svg:rect" -> "svg")
            if let Some((prefix, _)) = elem.name.split_once(':') {
                used.insert(prefix.to_string());
            }

            // Check attributes (e.g. "xlink:href" -> "xlink")
            for (key, _) in &elem.attributes {
                if let Some((prefix, _)) = key.split_once(':') {
                    // xml:space etc. "xml" is visible
                    used.insert(prefix.to_string());
                }
            }

            collect_used_prefixes(&elem.children, used);
        }
    }
}

fn process_root(nodes: &mut Vec<Node>, used: &HashSet<String>) {
    // Usually namespaces are defined on the first element (root svg)
    // But technically can be anywhere. Simplifying to check all elements for xmlns definition.

    for node in nodes {
        if let Node::Element(elem) = node {
            // Retain only used xmlns
            // "xmlns" default namespace is special, we usually allow it if it's SVG
            // "xmlns:prefix"

            let mut to_remove = Vec::new();

            for key in elem.attributes.keys() {
                if key.starts_with("xmlns:") {
                    let prefix = &key[6..];
                    if !used.contains(prefix) {
                        to_remove.push(key.clone());
                    }
                }
            }

            for k in to_remove {
                elem.attributes.shift_remove(&k);
            }

            // Recurse? Though unlikely to have nested xmlns definitions in optimized SVG
            process_root(&mut elem.children, used);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_remove_unused() {
        let input = "<svg xmlns:xlink=\"http://www.w3.org/1999/xlink\" xmlns:sketch=\"http://sketchapp.com\"><rect/></svg>";
        let expected = "<svg><rect/></svg>";

        let mut doc = parser::parse(input).unwrap();
        RemoveUnusedNS.apply(&mut doc);
        assert_eq!(printer::print(&doc), expected);
    }

    #[test]
    fn test_keep_used() {
        let input =
            "<svg xmlns:xlink=\"http://www.w3.org/1999/xlink\"><use xlink:href=\"#id\"/></svg>";
        let mut doc = parser::parse(input).unwrap();
        RemoveUnusedNS.apply(&mut doc);
        assert_eq!(printer::print(&doc), input);
    }
}
