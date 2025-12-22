use crate::tree::Document;

pub trait Plugin {
    fn apply(&self, doc: &mut Document);
}

pub mod remove_comments;

// Re-export specific plugins
pub use remove_comments::RemoveComments;
