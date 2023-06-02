mod node;

use std::ops::Range;

use crate::ScopeGate;

pub(crate) use self::node::Node;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocNode {
    expression_l: Loc,
    node: Node,
    scope_gate: ScopeGate,
}

/// Represents the beginning and end of a Node.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Loc {
    begin: usize,
    end: usize,
}

impl Loc {
    /// Offset of the start of the node.
    ///
    pub fn begin(&self) -> usize {
        self.begin
    }

    /// Offset of the end of the node.
    ///
    pub fn end(&self) -> usize {
        self.end
    }

    pub fn as_range(&self) -> Range<usize> {
        self.begin..self.end
    }

    pub fn contains(&self, other: &Loc) -> bool {
        self.begin <= other.begin && other.end <= self.end
    }
}

impl From<lib_ruby_parser::Loc> for Loc {
    #[inline]
    fn from(value: lib_ruby_parser::Loc) -> Self {
        Self {
            begin: value.begin,
            end: value.end,
        }
    }
}
