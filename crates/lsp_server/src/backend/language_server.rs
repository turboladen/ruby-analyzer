use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
        InitializeParams, InitializeResult, InitializedParams, MessageType, ServerCapabilities,
        ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
    },
    LanguageServer,
};
use tracing::debug;

use super::Backend;

const SERVER_CAPABILITIES: ServerCapabilities = ServerCapabilities {
    call_hierarchy_provider: None,
    code_action_provider: None,
    code_lens_provider: None,
    color_provider: None,
    completion_provider: None,
    declaration_provider: None,
    definition_provider: None,
    document_formatting_provider: None,
    document_highlight_provider: None,
    document_link_provider: None,
    document_on_type_formatting_provider: None,
    document_range_formatting_provider: None,
    document_symbol_provider: None,
    execute_command_provider: None,
    experimental: None,
    folding_range_provider: None,
    hover_provider: None,
    implementation_provider: None,
    inlay_hint_provider: None,
    inline_value_provider: None,
    linked_editing_range_provider: None,
    moniker_provider: None,
    position_encoding: None,
    references_provider: None,
    rename_provider: None,
    selection_range_provider: None,
    semantic_tokens_provider: None,
    signature_help_provider: None,
    text_document_sync: Some(TextDocumentSyncCapability::Kind(
        TextDocumentSyncKind::INCREMENTAL,
    )),
    type_definition_provider: None,
    workspace: None,
    workspace_symbol_provider: None,
};

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        debug!("Server initializing...");
        self.client()
            .log_message(MessageType::INFO, "Initializing server...")
            .await;

        *self.session.client_capabilities_mut().await = Some(params.capabilities);

        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: SERVER_CAPABILITIES,
            // offset_encoding: Some("utf-8".to_string()),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        debug!("Server initialized");

        self.client()
            .log_message(MessageType::INFO, "ruby_analyzer server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        debug!("Server shutdown");

        self.client()
            .log_message(MessageType::INFO, "ruby_analyzer server shut down")
            .await;

        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        debug!("Client opened file {}", params.text_document.uri);

        if let "ruby" = params.text_document.language_id.as_str() {
            self.session.open_ruby_document(params.text_document).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        debug!("Client closed file {}", params.text_document.uri);

        // We don't get an indication of which language the closed file was, so once we deal with
        // more than ruby files (i.e. rbs), the following code should update all the collections.
        // self.session.close_ruby_document(params.text_document.uri);
        self.session
            .ruby_document_open_states()
            .close(params.text_document.uri);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        debug!("Client changed file {}", params.text_document.uri);

        // We don't get an indication of which language the changed file was, so once we deal with
        // more than ruby files (i.e. rbs), the following code should update all the collections.
        self.session
            .change_ruby_document(params.text_document, params.content_changes);
    }
}
