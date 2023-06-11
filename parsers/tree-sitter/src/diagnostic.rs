use ropey::{Rope, RopeSlice};
use tree_sitter::Node;

/// Captures info about an `ERROR` or `MISSING` node from the parse tree.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic<'a> {
    pub(super) kind_id: u16,
    pub(super) diag_type: DiagType,
    pub(super) range: tree_sitter::Range,

    pub(super) kind: &'static str,
    pub(super) code: RopeSlice<'a>,
}

impl<'a> Diagnostic<'a> {
    pub(super) fn from_missing(value: Node<'_>, source: &'a Rope) -> Self {
        Self {
            kind_id: value.kind_id(),
            kind: value.kind(),
            diag_type: DiagType::Missing,
            range: value.range(),
            code: source.slice(value.start_byte()..value.end_byte()),
        }
    }

    #[must_use]
    pub const fn kind_id(&self) -> u16 {
        self.kind_id
    }

    #[must_use]
    pub const fn kind(&self) -> &str {
        self.kind
    }

    #[must_use]
    pub const fn diag_type(&self) -> DiagType {
        self.diag_type
    }

    #[must_use]
    pub const fn range(&self) -> tree_sitter::Range {
        self.range
    }

    #[must_use]
    pub const fn code(&self) -> &RopeSlice<'a> {
        &self.code
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagType {
    Error,
    Missing,
}
