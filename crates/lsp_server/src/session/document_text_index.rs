use std::{ops::Deref, sync::Arc};

use dashmap::DashMap;
use ropey::Rope;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::Url;
use tree_sitter::Point;

use crate::ropey_ext::Endings;

#[derive(Default, Debug, Clone)]
pub(crate) struct Document {
    version: i32,
    text: Rope,
}

#[derive(Default, Clone)]
pub(crate) struct DocumentTextIndex {
    inner: Arc<DashMap<Url, RwLock<Document>>>,
}

impl DocumentTextIndex {
    pub(crate) fn store_full_text(&self, uri: Url, version: i32, code: Rope) {
        self.inner
            .entry(uri)
            .and_modify(|rw_lock| {
                // TODO: This could maybe be more efficient if we use tree_sitter::Tree's `edit()`
                // and `changed_ranges()`.
                let mut document = rw_lock.blocking_write();

                document.text = code.clone();
            })
            .or_insert(RwLock::new(Document {
                version,
                text: code.clone(),
            }));
    }

    pub(crate) fn end_byte_and_point(&self, uri: &Url) -> Option<(usize, Point)> {
        let binding = self.inner.get(uri);
        let rw_lock = binding.as_deref()?;

        let doc = rw_lock.blocking_read();
        Some(doc.text.end_byte_and_point())
    }

    //     pub(crate) fn get<'a>(&'a self, uri: &Url) -> Option<&'a RwLock<Document>> {
    //         self.inner.get(uri).map(|dashref| dashref.value())
    //     }
}

impl Deref for DocumentTextIndex {
    type Target = Arc<DashMap<Url, RwLock<Document>>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
