mod icky_nodes;

use ropey::Rope;
use tracing::debug;
use tree_sitter::{Node, Query, QueryCursor, Tree, TreeCursor};
use tree_sitter_ruby::language;

use crate::diagnostic::{DiagType, Diagnostic};

use self::icky_nodes::IckyNodes;

/// The output from `crate::parser::parse()`. It contains both the resulting `Tree` and the code
/// that was parsed; it also provides a way to get diagnostics (these are only gathered on-demand,
/// instead of doing while instantiating the `ParseResult`).
///
#[derive(Debug, Clone)]
pub struct ParseResult<'a> {
    tree: Tree,
    code: &'a Rope,
}

impl<'a> ParseResult<'a> {
    #[must_use]
    pub fn new(tree: Tree, code: &'a Rope) -> Self {
        Self { tree, code }
    }

    /// Inspects the internal `Tree` for `ERROR` and `MISSING` nodes, then turns those into
    /// `Diagnostic`s.
    ///
    #[must_use]
    pub fn diagnostics(&self) -> Vec<Diagnostic<'_>> {
        let missing = {
            let cursor = self.tree.walk();
            extract_missing_nodes(cursor)
        };
        let errors = extract_error_nodes(self.tree.root_node(), self.code);

        IckyNodes::new(errors, missing).into_diags(self.code)
    }

    #[must_use]
    pub const fn tree(&self) -> &Tree {
        &self.tree
    }

    #[must_use]
    pub const fn code(&self) -> &Rope {
        self.code
    }
}

fn extract_missing_nodes(mut cursor: TreeCursor<'_>) -> Vec<Node<'_>> {
    fn process_siblings<'a>(
        mut cursor: TreeCursor<'a>,
        missing_nodes: &mut Vec<Node<'a>>,
    ) -> TreeCursor<'a> {
        let mut has_siblings = true;

        while has_siblings {
            let node = cursor.node();

            if node.is_missing() {
                missing_nodes.push(node);
            }

            has_siblings = cursor.goto_next_sibling();
        }

        cursor
    }

    fn process_children<'a>(mut cursor: TreeCursor<'a>, missing_nodes: &mut Vec<Node<'a>>) {
        let mut has_child = true;

        while has_child {
            let node = cursor.node();

            if node.is_missing() {
                missing_nodes.push(node);
            }

            cursor = process_siblings(cursor, missing_nodes);
            has_child = cursor.goto_first_child();
        }
    }

    let mut missing_nodes = Vec::new();

    cursor = process_siblings(cursor, &mut missing_nodes);
    process_children(cursor, &mut missing_nodes);

    missing_nodes
}

fn extract_error_nodes<'a>(root_node: Node<'a>, code: &'a Rope) -> Vec<Diagnostic<'a>> {
    let query = Query::new(language(), r#"(ERROR) @error"#).expect("Invalid query!");
    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, root_node, |node: Node<'_>| {
        code.byte_slice(node.byte_range())
            .chunks()
            .map(str::as_bytes)
    });

    // get the index of the capture named "error"
    let raise_idx = query.capture_index_for_name("error").unwrap();

    let mut diags = Vec::new();

    for each_match in matches {
        // iterate over all captures called "error"
        // ignore captures such as "fn-name"
        for capture in each_match.captures.iter().filter(|c| c.index == raise_idx) {
            let range = capture.node.range();

            let text = code.slice(range.start_byte..range.end_byte);
            let line = range.start_point.row;
            let col = range.start_point.column;
            debug!(
                "[Line: {}, Col: {}] Offending source code: `{}`",
                line, col, text
            );
            diags.push(Diagnostic {
                kind_id: capture.node.kind_id(),
                kind: capture.node.kind(),
                diag_type: DiagType::Error,
                range,
                code: text,
            });
        }
    }

    diags
}
