use std::sync::Arc;

use dashmap::DashMap;
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
    pub(crate) fn open(&self, uri: Url) {
        self.inner
            .entry(uri)
            .and_modify(|rw_lock| {
                let mut state = rw_lock.blocking_write();
                *state = DocumentOpenState::Open;
            })
            .or_insert(RwLock::new(DocumentOpenState::Open));
    }

    pub(crate) fn close(&self, uri: Url) {
        self.inner
            .entry(uri)
            .and_modify(|rw_lock| {
                let mut state = rw_lock.blocking_write();
                *state = DocumentOpenState::Closed;
            })
            .or_insert(RwLock::new(DocumentOpenState::Closed));
    }
}
