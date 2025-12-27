use crate::plugins::collections::find_used_ids;
use crate::plugins::Plugin;
use crate::tree::{Document, Node};
use std::collections::HashSet;

pub struct RemoveUselessDefs;

impl Plugin for RemoveUselessDefs {
    fn apply(&self, doc: &mut Document) {
        let mut used_ids = HashSet::new();
        for node in &doc.root {
            find_used_ids(node, &mut used_ids);
        }

        remove_useless_defs_in_nodes(&mut doc.root, &used_ids);
    }
}

fn remove_useless_defs_in_nodes(nodes: &mut Vec<Node>, used_ids: &HashSet<String>) {
    // 1. Recurse first (to clean nested defs)
    for node in nodes.iter_mut() {
        if let Node::Element(elem) = node {
            remove_useless_defs_in_nodes(&mut elem.children, used_ids);

            // If there is ANY defs element, filter its children now (mutable access)
            if elem.name == "defs" {
                elem.children.retain(|child| {
                    if let Node::Element(child_elem) = child {
                        // Keep if it has an ID and that ID is used
                        if let Some(id) = child_elem.attributes.get("id") {
                            if used_ids.contains(id) {
                                return true;
                            }
                        }
                        // Remove unused definitions
                        return false;
                    }
                    // Remove non-element nodes in defs (text, comments)
                    false
                });
            }
        }
    }

    // 2. Remove empty defs (retain)
    nodes.retain(|node| {
        if let Node::Element(elem) = node {
            if elem.name == "defs" && elem.children.is_empty() {
                return false;
            }
        }
        true
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_remove_useless_defs() {
        let input =
            "<svg><defs><rect id=\"unused\"/><rect id=\"used\"/></defs><use href=\"#used\"/></svg>";
        let expected = "<svg><defs><rect id=\"used\"/></defs><use href=\"#used\"/></svg>";

        let mut doc = parser::parse(input).unwrap();
        RemoveUselessDefs.apply(&mut doc);
        let output = printer::print(&doc);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_remove_empty_defs() {
        let input = "<svg><defs><rect id=\"unused\"/></defs></svg>";
        let expected = "<svg/>";

        let mut doc = parser::parse(input).unwrap();
        RemoveUselessDefs.apply(&mut doc);
        let output = printer::print(&doc);
        assert_eq!(output, expected);
    }
}
