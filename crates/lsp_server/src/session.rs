use std::sync::Arc;

use dashmap::DashMap;
use ropey::Rope;
use ruby_analyzer_tree_sitter_parser::{
    diagnostic::{DiagType, Diagnostic as RaDiagnostic},
    parser::parse,
};
use tokio::sync::{RwLock, RwLockWriteGuard};
use tower_lsp::{
    lsp_types::{
        self, ClientCapabilities, Diagnostic as LspDiagnostic, DiagnosticSeverity,
        TextDocumentItem, Url,
    },
    Client,
};
use tracing::debug;

pub(crate) struct Session {
    client: Client,
    client_capabilities: RwLock<Option<ClientCapabilities>>,

    /// Keep track that the client has the doc open. If the file is open, we can't make changes on
    /// the server side.
    pub(crate) ruby_document_open_states: Arc<DashMap<Url, DocumentOpenState>>,
    pub(crate) ruby_document_texts: Arc<DashMap<Url, RwLock<Rope>>>,
    pub(crate) ruby_document_trees: Arc<DashMap<Url, RwLock<tree_sitter::Tree>>>,
}

impl Session {
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            client_capabilities: RwLock::default(),
            ruby_document_open_states: Default::default(),
            ruby_document_texts: Default::default(),
            ruby_document_trees: Default::default(),
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
        {
            self.ruby_document_open_states
                .entry(text_document_item.uri.clone())
                .and_modify(|state| {
                    *state = DocumentOpenState::Open;
                })
                .or_insert(DocumentOpenState::Open);

            debug!(
                "ruby_document_open_states: {:#?}",
                self.ruby_document_open_states
            );
        }

        let code = Rope::from_str(&text_document_item.text);

        // Store the file->code relationship.
        {
            self.ruby_document_texts
                .entry(text_document_item.uri.clone())
                .and_modify(|rw_lock| {
                    // TODO: This could maybe be more efficient if we use tree_sitter::Tree's `edit()`
                    // and `changed_ranges()`.
                    let mut rope = rw_lock.blocking_write();

                    *rope = code.clone();
                })
                .or_insert(RwLock::new(code.clone()));
        }

        // TODO: I don't think `parse()` needs to care about returning anything else but the Tree.
        let parse_result = match parse(&code, None) {
            Some(t) => t,
            None => {
                todo!("Check what None means and handle that")
            }
        };

        // Store the file->tree relationship.
        self.ruby_document_trees
            .entry(text_document_item.uri.clone())
            .and_modify(|rw_lock| {
                let mut tree = rw_lock.blocking_write();

                *tree = parse_result.tree().clone();
            })
            .or_insert(RwLock::new(parse_result.tree().clone()));

        let diagnostics = parsed_diags_to_lsp_diags(parse_result.diagnostics());

        self.client
            .publish_diagnostics(
                text_document_item.uri,
                diagnostics,
                Some(text_document_item.version),
            )
            .await;
    }

    pub(crate) async fn close_ruby_document(&self, uri: Url) {
        self.ruby_document_open_states
            .entry(uri)
            .and_modify(|state| {
                *state = DocumentOpenState::Closed;
            })
            .or_insert(DocumentOpenState::Closed);

        debug!(
            "ruby_document_open_states: {:#?}",
            self.ruby_document_open_states
        );
    }
}

fn lsp_range_from_ts_range(ts_range: tree_sitter::Range) -> lsp_types::Range {
    lsp_types::Range {
        start: lsp_types::Position {
            line: u32::try_from(ts_range.start_point.row).unwrap(),
            character: u32::try_from(ts_range.start_point.column).unwrap(),
        },
        end: lsp_types::Position {
            line: u32::try_from(ts_range.end_point.row).unwrap(),
            character: u32::try_from(ts_range.end_point.column).unwrap(),
        },
    }
}

fn parsed_diags_to_lsp_diags(diagnostics: Vec<RaDiagnostic<'_>>) -> Vec<LspDiagnostic> {
    const SOURCE: &str = "ruby_analyzer";

    diagnostics
        .into_iter()
        .map(|ts_diag| {
            let (severity, message) = match ts_diag.diag_type() {
                DiagType::Error => {
                    let msg = format!("`{}` error", ts_diag.kind());

                    (DiagnosticSeverity::ERROR, msg)
                }
                DiagType::Missing => {
                    let msg = format!("`{}` missing", ts_diag.kind());
                    (DiagnosticSeverity::WARNING, msg)
                }
            };
            LspDiagnostic {
                range: lsp_range_from_ts_range(ts_diag.range()),
                severity: Some(severity),
                code: None,
                code_description: None,
                source: Some(SOURCE.to_string()),
                message,
                related_information: None,
                tags: None,
                data: None,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum DocumentOpenState {
    Open,
    Closed,
}
