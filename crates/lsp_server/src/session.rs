mod document;
mod document_open_state_index;
mod document_text_index;
mod parse_tree_index;

use ropey::Rope;
use tokio::sync::{RwLock, RwLockWriteGuard};
use tower_lsp::{
    lsp_types::{
        ClientCapabilities, TextDocumentContentChangeEvent, TextDocumentItem,
        VersionedTextDocumentIdentifier,
    },
    Client,
};
use tracing::{debug, trace};

use crate::ext_traits::Endings;

use self::{
    document_open_state_index::DocumentOpenStateIndex, document_text_index::DocumentTextIndex,
    parse_tree_index::ParseTreeIndex,
};

/// A `Session` holds the state of the server.
///
pub(crate) struct Session {
    client: Client,
    client_capabilities: RwLock<Option<ClientCapabilities>>,

    pub(crate) ruby_document_open_states: DocumentOpenStateIndex,
    pub(crate) ruby_document_texts: DocumentTextIndex,
    pub(crate) ruby_parse_trees: ParseTreeIndex,
}

impl Session {
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            client_capabilities: RwLock::default(),
            ruby_document_open_states: Default::default(),
            ruby_document_texts: Default::default(),
            ruby_parse_trees: Default::default(),
        }
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    /// Mutable accessor for updating `ClientCapabilities`.
    ///
    pub(crate) async fn client_capabilities_mut(
        &self,
    ) -> RwLockWriteGuard<Option<ClientCapabilities>> {
        self.client_capabilities.write().await
    }

    /// Implementation details for handling `LanguageServer::did_open()`.
    ///
    pub(crate) async fn open_ruby_document(&self, text_document_item: TextDocumentItem) {
        // Keep track that the client has the doc open.
        self.ruby_document_open_states
            .open(text_document_item.uri.clone())
            .await;

        let code = Rope::from_str(&text_document_item.text);

        // Store the file->code relationship.
        self.ruby_document_texts
            .store_full_text(
                text_document_item.uri.clone(),
                text_document_item.version,
                code.clone(),
            )
            .await;

        // TODO: Ideally, this is only computed if the `do_full_parse` call needs it,
        // but it needs to be async and I'm not sure yet how to pass an async closure to a function
        // (can you??)
        let end_byte_and_point = self
            .ruby_document_texts
            .end_byte_and_point(&text_document_item.uri)
            .await
            .unwrap_or_default();

        // Store the file->tree relationship.
        let diagnostics = self
            .ruby_parse_trees
            .do_full_parse(
                text_document_item.uri.clone(),
                &code,
                end_byte_and_point,
                &self.client,
            )
            .await;

        debug!("Got diagnostics during didOpen: {:?}", &diagnostics);

        // NOTE: Sending these even if they're empty is good; it's the server's job for maintaining
        // the state of diagnostics tied to a file. Once the server sends them, the client renders
        // them, the server has to tell the client that the diagnostics can be cleared.
        self.client
            .publish_diagnostics(
                text_document_item.uri,
                diagnostics,
                Some(text_document_item.version),
            )
            .await;
    }

    /// Implementation details for handling `LanguageServer::did_change()`.
    ///
    pub(crate) async fn change_ruby_document(
        &self,
        identifier: VersionedTextDocumentIdentifier,
        content_changes: Vec<TextDocumentContentChangeEvent>,
    ) {
        self.ruby_document_texts
            .change_doc(&identifier.uri, identifier.version, &content_changes)
            .await;

        trace!("[didChange] Done updating text");

        let code = self
            .ruby_document_texts
            .get_latest_code_unchecked(&identifier.uri)
            .await;

        let diagnostics = self
            .ruby_parse_trees
            .do_full_parse(
                identifier.uri.clone(),
                &code,
                code.end_byte_and_point(),
                &self.client,
            )
            .await;

        debug!("Got diagnostics during didChange: {:?}", &diagnostics);

        self.client
            .publish_diagnostics(identifier.uri, diagnostics, Some(identifier.version))
            .await;
    }

    pub(crate) fn ruby_document_open_states(&self) -> &DocumentOpenStateIndex {
        &self.ruby_document_open_states
    }
}
