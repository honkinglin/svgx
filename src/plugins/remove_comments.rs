use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct RemoveComments;

impl Plugin for RemoveComments {
    fn apply(&self, doc: &mut Document) {
        remove_comments_from_nodes(&mut doc.root);
    }
}

fn remove_comments_from_nodes(nodes: &mut Vec<Node>) {
    nodes.retain(|node| !matches!(node, Node::Comment(_)));
    for node in nodes {
        if let Node::Element(elem) = node {
            remove_comments_from_nodes(&mut elem.children);
        }
    }
}
