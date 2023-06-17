use std::{ops::Deref, sync::Arc};

use dashmap::{mapref::entry::Entry, DashMap};
use ropey::Rope;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::Url;
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
}

impl Deref for DocumentTextIndex {
    type Target = Arc<DashMap<Url, RwLock<Document>>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
