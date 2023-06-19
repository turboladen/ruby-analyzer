use std::sync::Arc;

use dashmap::{mapref::entry::Entry, DashMap};
use ropey::Rope;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{TextDocumentContentChangeEvent, Url};
use tree_sitter::Point;

use crate::ext_traits::Endings;

use super::document::Document;

/// Essentially indexes filename->code.
///
#[derive(Default, Clone)]
pub(crate) struct DocumentTextIndex {
    inner: Arc<DashMap<Url, RwLock<Document>>>,
}

impl DocumentTextIndex {
    /// If a file at `uri` is already indexed, this just updates the `version` and sets the `code`
    /// with the given value. If a file at `uri` is _not_ already indexed, this just inserts a new
    /// `Document` with the given data.
    ///
    pub(crate) async fn store_full_text(&self, uri: Url, version: i32, code: Rope) {
        match self.inner.entry(uri) {
            Entry::Occupied(entry) => {
                let mut document = entry.get().write().await;

                document.version = version;
                document.code = code;
            }
            Entry::Vacant(entry) => {
                entry.insert(RwLock::new(Document {
                    version,
                    code: code.clone(),
                }));
            }
        }
    }

    /// Wrapper method for getting the end byte and `Point` from the file at `uri`. Returns `None`
    /// if there's no file already indexed for `uri`.
    ///
    pub(crate) async fn end_byte_and_point(&self, uri: &Url) -> Option<(usize, Point)> {
        let binding = self.inner.get(uri);
        let rw_lock = binding.as_deref()?;
        let doc = rw_lock.read().await;

        Some(doc.code.end_byte_and_point())
    }

    pub(crate) async fn change_doc(
        &self,
        uri: &Url,
        version: i32,
        content_changes: &[TextDocumentContentChangeEvent],
    ) {
        match self.inner.get(uri) {
            Some(rw_lock) => {
                let mut doc = rw_lock.write().await;

                if version <= doc.version {
                    // TODO: Handle more gracefully.
                    panic!("Got update for older doc");
                }

                for content_change in content_changes {
                    match content_change.range {
                        Some(range) => {
                            doc.merge_for_change_unchecked(version, &range, &content_change.text);
                        }
                        None => {
                            // Set the existing rope to entry.
                            doc.replace_for_change_unchecked(version, &content_change.text);
                        }
                    }
                }
            }
            None => {
                // TODO: Probably should handle this by just storing the things as if they were new?
                panic!("Got change notices for unknown doc");
            }
        }
    }

    /// Assumes there is a value for `uri` and returns the document's code.
    ///
    pub(crate) async fn get_latest_code_unchecked(&self, uri: &Url) -> Rope {
        let it = self
            .inner
            .get(uri)
            .expect("Don't call this unless you know there's a value already");
        let doc = it.read().await;

        doc.code.clone()
    }
}
