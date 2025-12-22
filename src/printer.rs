use crate::tree::{Document, Node};

pub fn print(doc: &Document) -> String {
    let mut out = String::new();
    for node in &doc.root {
        print_node(node, &mut out);
    }
    out
}

fn print_node(node: &Node, out: &mut String) {
    match node {
        Node::Element(elem) => {
            out.push('<');
            out.push_str(&elem.name);
            for (k, v) in &elem.attributes {
                out.push(' ');
                out.push_str(k);
                out.push_str("=\"");
                out.push_str(v);
                out.push('"');
            }

            if elem.children.is_empty() {
                out.push_str("/>");
            } else {
                out.push('>');
                for child in &elem.children {
                    print_node(child, out);
                }
                out.push_str("</");
                out.push_str(&elem.name);
                out.push('>');
            }
        }
        Node::Text(text) => {
            out.push_str(text);
        }
        Node::Comment(text) => {
            out.push_str("<!--");
            out.push_str(text);
            out.push_str("-->");
        }
        Node::Cdata(text) => {
            out.push_str("<![CDATA[");
            out.push_str(text);
            out.push_str("]]>");
        }
        Node::ProcessingInstruction(target, content) => {
            out.push_str("<?");
            out.push_str(target);
            if let Some(c) = content {
                out.push(' ');
                out.push_str(c);
            }
            out.push_str("?>");
        }
        Node::Doctype(text) => {
            out.push_str("<!DOCTYPE ");
            out.push_str(text);
            out.push_str(">");
        }
    }
}
