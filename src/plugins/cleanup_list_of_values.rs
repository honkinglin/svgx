use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct CleanupListOfValues {
    pub float_precision: usize,
    pub leading_zero: bool,
    pub default_px: bool,
    pub convert_to_px: bool,
    // Target lists: viewBox, points, dx, dy, x, y, rotate (list), enable-background
}

impl Default for CleanupListOfValues {
    fn default() -> Self {
        Self {
            float_precision: 3,
            leading_zero: true,
            default_px: true,
            convert_to_px: true,
        }
    }
}

impl Plugin for CleanupListOfValues {
    fn apply(&self, doc: &mut Document) {
        process_lists(&mut doc.root, self);
    }
}

fn process_lists(nodes: &mut Vec<Node>, opts: &CleanupListOfValues) {
    let list_attrs = ["viewBox", "points", "dx", "dy", "x", "y", "rotate"];

    for node in nodes {
        if let Node::Element(elem) = node {
            for attr in list_attrs {
                if let Some(val) = elem.attributes.get_mut(attr) {
                    let clean = cleanup_list(val, opts);
                    *val = clean;
                }
            }
            process_lists(&mut elem.children, opts);
        }
    }
}

fn cleanup_list(val: &str, opts: &CleanupListOfValues) -> String {
    // Split by comma or space
    // SVG lists can use both. "10, 20" or "10 20"
    // Normalize to spaces? Or commas? SVGO prefers spaces usually (fewer chars than ", ").

    let mut clean_str = String::new();
    let parts = val.replace(',', " ");

    for part in parts.split_whitespace() {
        if !clean_str.is_empty() {
            clean_str.push(' ');
        }

        // Process number
        // Simple numeric cleanup logic duplicated from cleanup_numeric_values?
        // Should use a shared helper.
        // For now, minimal impl.

        if let Ok(num) = part.trim_end_matches("px").parse::<f64>() {
            let p = opts.float_precision;
            let factor = 10u32.pow(p as u32) as f64;
            let rounded = (num * factor).round() / factor;
            let mut s = rounded.to_string();

            if opts.leading_zero {
                if s.starts_with("0.") {
                    s = s[1..].to_string();
                } else if s.starts_with("-0.") {
                    s = format!("-{}", &s[2..]);
                }
            }
            clean_str.push_str(&s);
        } else {
            clean_str.push_str(part);
        }
    }

    clean_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_viewbox() {
        let opts = CleanupListOfValues::default();
        let val = "0 0 100.500 100.1234";
        let out = cleanup_list(val, &opts);
        assert_eq!(out, "0 0 100.5 100.123");
    }

    #[test]
    fn test_cleanup_points() {
        let opts = CleanupListOfValues::default();
        let val = "0,0 100,0 100,100"; // commans
        let out = cleanup_list(val, &opts);
        assert_eq!(out, "0 0 100 0 100 100");
    }
}
