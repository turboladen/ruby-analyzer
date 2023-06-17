use std::sync::Arc;

use dashmap::{mapref::entry::Entry, DashMap};
use tokio::sync::RwLock;
use tower_lsp::lsp_types::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum DocumentOpenState {
    Open,
    Closed,
}

/// Keep track that the client has the doc open. If the file is open, we can't make changes on
/// the server side.
#[derive(Default, Clone)]
pub(crate) struct DocumentOpenStateIndex {
    inner: Arc<DashMap<Url, RwLock<DocumentOpenState>>>,
}

impl DocumentOpenStateIndex {
    pub(crate) async fn open(&self, uri: Url) {
        match self.inner.entry(uri) {
            Entry::Occupied(entry) => {
                let mut state = entry.get().write().await;
                *state = DocumentOpenState::Open;
            }
            Entry::Vacant(entry) => {
                entry.insert(RwLock::new(DocumentOpenState::Open));
            }
        }
    }

    pub(crate) async fn close(&self, uri: Url) {
        match self.inner.entry(uri) {
            Entry::Occupied(entry) => {
                let mut state = entry.get().write().await;
                *state = DocumentOpenState::Closed;
            }
            Entry::Vacant(entry) => {
                entry.insert(RwLock::new(DocumentOpenState::Closed));
            }
        }
    }
}
