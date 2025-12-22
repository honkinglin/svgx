use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct RemoveDoctype;

impl Plugin for RemoveDoctype {
    fn apply(&self, doc: &mut Document) {
        doc.root.retain(|node| !matches!(node, Node::Doctype(_)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_remove_doctype() {
        let input = "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\"><svg></svg>";
        let expected = "<svg/>";

        let mut doc = parser::parse(input).unwrap();
        RemoveDoctype.apply(&mut doc);
        let output = printer::print(&doc);

        assert_eq!(output, expected);
    }
}
