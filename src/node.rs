use lib_ruby_parser::Loc;
use lsp_types::Position;

use crate::{namespace::Namespace, properties::Properties};

/// A `Node` represents an item in a ruby `Ast`. Unlike `lib-ruby-parser`'s Ast, which represents
/// a node's hierarchy as part of the node itself (ex. a `class` node contains all of its `def`
/// nodes), this `Node` contains an `id` and a representation of the Ruby scope it lives in (the
/// `scope_name_branch`), which, when put in the context of needing to look up where this node
/// resides in an entire codebase, lends itself more toward lookup in `Vec`-based index (which is
/// the pattern we use), as opposed to having to traverse a tree or graph.
///
#[derive(Debug)]
pub struct Node {
    pub(crate) id: usize,
    pub(crate) namespace: Namespace,
    pub(crate) starting_position: Position,
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

    pub const fn starting_position(&self) -> Position {
        self.starting_position
    }

    pub const fn expression_l(&self) -> &Loc {
        &self.expression_l
    }

    pub const fn properties(&self) -> &Properties {
        &self.properties
    }

    // pub fn scope_name_branch_for_self(&self) -> Option<Namespace> {
    //     match &self.properties {
    //         Properties::Class(class) => Some(
    //             self.scope_name_branch
    //                 .join(NamespaceNode::Class(class.name.clone())),
    //         ),
    //         _ => None,
    //     }
    // }
}
