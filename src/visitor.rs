use crate::tree::{Document, Element, Node};

pub trait Visitor {
    fn visit_document(&mut self, doc: &mut Document) {
        self.visit_nodes(&mut doc.root);
    }

    fn visit_nodes(&mut self, nodes: &mut Vec<Node>) {
        for node in nodes {
            self.visit_node(node);
        }
    }

    fn visit_node(&mut self, node: &mut Node) {
        if let Node::Element(elem) = node {
            self.visit_element(elem);
        }
    }

    fn visit_element(&mut self, element: &mut Element) {
        self.visit_nodes(&mut element.children);
    }
}
