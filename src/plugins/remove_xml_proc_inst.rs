use crate::plugins::Plugin;
use crate::tree::{Document, Node};

pub struct RemoveXMLProcInst;

impl Plugin for RemoveXMLProcInst {
    fn apply(&self, doc: &mut Document) {
        doc.root.retain(
            |node| !matches!(node, Node::ProcessingInstruction(target, _) if target == "xml"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::printer;

    #[test]
    fn test_remove_xml_proc_inst() {
        let input = "<?xml version=\"1.0\"?><svg></svg>";
        let expected = "<svg/>";

        let mut doc = parser::parse(input).unwrap();
        RemoveXMLProcInst.apply(&mut doc);
        let output = printer::print(&doc);

        assert_eq!(output, expected);
    }
}
