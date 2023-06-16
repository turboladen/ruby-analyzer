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

use self::{
    document_open_state_index::DocumentOpenStateIndex, document_text_index::DocumentTextIndex,
    parse_tree_index::ParseTreeIndex,
};

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
            .open(text_document_item.uri.clone());

        let code = Rope::from_str(&text_document_item.text);

        // Store the file->code relationship.
        self.ruby_document_texts.store_full_text(
            text_document_item.uri.clone(),
            text_document_item.version,
            code.clone(),
        );

        // // Store the file->tree relationship.
        let diagnostics =
            self.ruby_parse_trees
                .do_full_parse(text_document_item.uri.clone(), &code, || {
                    self.ruby_document_texts
                        .end_byte_and_point(&text_document_item.uri)
                        .unwrap_or_default()
                });

        self.client
            .publish_diagnostics(
                text_document_item.uri,
                diagnostics,
                Some(text_document_item.version),
            )
            .await;
    }

    pub(crate) fn change_ruby_document(
        &self,
        identifier: VersionedTextDocumentIdentifier,
        content_changes: Vec<TextDocumentContentChangeEvent>,
    ) {
        match self.ruby_document_texts.get(&identifier.uri) {
            Some(rw_lock) => {
                let mut doc = rw_lock.blocking_write();

                for content_change in content_changes {
                    match content_change.range {
                        Some(range) => {
                            todo!("Update the existing Rope and parse tree");
                        }
                        None => {
                            // Set the existing rope to
                            // entry.
                        }
                    }
                }
            }
            None => todo!(),
        }
    }

    pub(crate) fn ruby_document_open_states(&self) -> &DocumentOpenStateIndex {
        &self.ruby_document_open_states
    }
}
