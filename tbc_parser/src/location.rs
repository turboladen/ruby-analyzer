mod node;

use std::ops::Range;

use crate::ScopeGate;

pub(crate) use self::node::NodeType;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocNode {
    pub(crate) node: NodeType,
    pub(crate) name: String,
    pub(crate) scope_gate: ScopeGate,
    pub(crate) expression_l: Loc,
}

impl LocNode {
    pub fn node(&self) -> NodeType {
        self.node
    }

    pub fn scope_gate(&self) -> &ScopeGate {
        &self.scope_gate
    }

    pub fn expression_l(&self) -> Loc {
        self.expression_l
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

pub trait Contains<T> {
    fn contains(&self, other: T) -> bool;
}

impl<'a> Contains<&'a Loc> for Loc {
    fn contains(&self, other: &Self) -> bool {
        self.begin <= other.begin && other.end <= self.end
    }
}

impl Contains<usize> for Loc {
    fn contains(&self, offset: usize) -> bool {
        self.begin <= offset && offset <= self.end
    }
}

/// Represents the beginning and end of a Node.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Loc {
    pub(crate) begin: usize,
    pub(crate) end: usize,
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
