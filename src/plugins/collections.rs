use crate::tree::Node;
use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

pub fn find_used_ids(node: &Node, used_ids: &mut HashSet<String>) {
    match node {
        Node::Element(elem) => {
            // Check all attributes for references
            for (_, value) in &elem.attributes {
                extract_ids_from_value(value, used_ids);
            }

            // Recurse
            for child in &elem.children {
                find_used_ids(child, used_ids);
            }
        }
        _ => {}
    }
}

fn extract_ids_from_value(value: &str, used_ids: &mut HashSet<String>) {
    // 1. url(#id)
    static URL_RE: OnceLock<Regex> = OnceLock::new();
    let url_re = URL_RE.get_or_init(|| Regex::new(r"url\s*\(\s*#([^\s\)]+)\s*\)").unwrap());

    for cap in url_re.captures_iter(value) {
        if let Some(id) = cap.get(1) {
            used_ids.insert(id.as_str().to_string());
        }
    }

    // 2. href="#id" (xlink:href or href)
    if value.starts_with('#') && value.len() > 1 {
        used_ids.insert(value[1..].to_string());
    }
}
