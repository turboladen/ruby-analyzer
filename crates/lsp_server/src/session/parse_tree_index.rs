use std::sync::Arc;

use dashmap::{mapref::entry::Entry, DashMap};
use ropey::Rope;
use ruby_analyzer_tree_sitter_parser::{
    diagnostic::{DiagType, Diagnostic as RaDiagnostic},
    parser::parse,
};
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{self, Diagnostic as LspDiagnostic, DiagnosticSeverity, Url};
use tree_sitter::{InputEdit, Point, Tree};

use crate::ext_traits::{Endings, FromTs};

/// Keeps the parse trees for each file we've parsed.
///
#[derive(Default, Clone)]
pub(crate) struct ParseTreeIndex {
    inner: Arc<DashMap<Url, RwLock<Tree>>>,
}

impl ParseTreeIndex {
    /// If the file at `uri` hasn't yet been parsed, it parses that and inserts it into the index.
    /// If the file at `uri` has been parsed, it uses `position_getter()` to determine the end
    /// positions (end byte, end position) of the latest code/text so we can incrementally update
    /// the existing `Tree` that we already have.
    ///
    pub(crate) async fn do_full_parse(
        &self,
        uri: Url,
        code: &Rope,
        old_positions: (usize, Point),
    ) -> Vec<LspDiagnostic> {
        match self.inner.entry(uri.clone()) {
            Entry::Occupied(entry) => {
                let (old_end_byte, old_end_position) = old_positions;
                let (new_end_byte, new_end_position) = code.end_byte_and_point();

                let input_edit = InputEdit {
                    start_byte: 0,
                    old_end_byte,
                    new_end_byte,
                    start_position: Point::new(0, 0),
                    old_end_position,
                    new_end_position,
                };

                let mut old_tree = entry.get().write().await;
                old_tree.edit(&input_edit);

                // TODO: I don't think `parse()` needs to care about returning anything else but the Tree.
                let parse_result = match parse(&code, Some(&old_tree)) {
                    Some(t) => t,
                    None => {
                        // https://docs.rs/tree-sitter/latest/tree_sitter/struct.Parser.html#method.parse
                        todo!("Tell the client we timed out or got cancelled.")
                    }
                };

                *old_tree = parse_result.tree().clone();
                parsed_diags_to_lsp_diags(parse_result.diagnostics())
            }
            Entry::Vacant(entry) => {
                let parse_result = match parse(&code, None) {
                    Some(t) => t,
                    None => {
                        // https://docs.rs/tree-sitter/latest/tree_sitter/struct.Parser.html#method.parse
                        todo!("Tell the client we timed out or got cancelled.")
                    }
                };

                entry.insert(RwLock::new(parse_result.tree().clone()));
                parsed_diags_to_lsp_diags(parse_result.diagnostics())
            }
        }
    }
}

/// Takes the diagnostics we get from the parser and turns them into `lsp_types::Diagnostic`s.
///
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
                range: lsp_types::Range::from_ts(ts_diag.range()),
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
