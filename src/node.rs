use crate::{namespace::Namespace, properties::Properties};

/// A `Node` represents an item in a ruby `Ast`. Unlike `lib-ruby-parser`'s Ast, which represents
/// a node's hierarchy as part of the node itself (ex. a `class` node contains all of its `def`
/// nodes), this `Node` contains an `id` and a representation of the Ruby scope it lives in (the
/// `scope_name_branch`), which, when put in the context of needing to look up where this node
/// resides in an entire codebase, lends itself more toward lookup in `Vec`-based index (which is
/// the pattern we use), as opposed to having to traverse a tree or graph.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node {
    pub(crate) id: usize,
    pub(crate) namespace: Namespace,
    // TODO: I think this doesn't belong here; and maybe isn't necessary at all (i.e. it could be
    // calculated using the cursor position and the rope, as needed, instead of for everything).
    pub(crate) expression_l: Loc,
    pub(crate) properties: Properties,
}

impl Node {
    pub const fn id(&self) -> usize {
        self.id
    }

    pub const fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub const fn expression_l(&self) -> &Loc {
        &self.expression_l
    }

    pub const fn properties(&self) -> &Properties {
        &self.properties
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Loc {
    begin: usize,
    end: usize,
}

impl Loc {
    pub fn begin(&self) -> usize {
        self.begin
    }

    pub fn end(&self) -> usize {
        self.end
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
