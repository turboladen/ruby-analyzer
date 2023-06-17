use std::{ops::Deref, sync::Arc};

use dashmap::{mapref::entry::Entry, DashMap};
use ropey::Rope;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{self, Url};
use tree_sitter::Point;

use crate::ropey_ext::{Endings, GetCharRange};

#[derive(Default, Debug, Clone)]
pub(crate) struct Document {
    pub(super) version: i32,
    pub(super) code: Rope,
}

impl Document {
    pub(super) fn replace_for_change_unchecked(&mut self, version: i32, text: &str) {
        self.version = version;
        self.code = Rope::from_str(text);
    }

    pub(super) fn merge_for_change_unchecked(
        &mut self,
        version: i32,
        range: &lsp_types::Range,
        new_text: &str,
    ) {
        self.version = version;

        let char_range = self.code.get_char_range(range);
        let start = char_range.start;
        self.code.remove(char_range);
        self.code.insert(start, new_text);
    }

    pub(crate) fn code(&self) -> &Rope {
        &self.code
    }
}

#[derive(Default, Clone)]
pub(crate) struct DocumentTextIndex {
    inner: Arc<DashMap<Url, RwLock<Document>>>,
}

impl DocumentTextIndex {
    pub(crate) async fn store_full_text(&self, uri: Url, version: i32, code: Rope) {
        match self.inner.entry(uri) {
            Entry::Occupied(entry) => {
                let mut document = entry.get().write().await;

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

    pub(crate) async fn end_byte_and_point(&self, uri: &Url) -> Option<(usize, Point)> {
        let binding = self.inner.get(uri);
        let rw_lock = binding.as_deref()?;
        let doc = rw_lock.read().await;

        Some(doc.code.end_byte_and_point())
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
