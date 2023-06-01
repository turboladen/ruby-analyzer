pub(crate) mod compat;
pub mod db;
pub(crate) mod lrp_extensions;
pub(crate) mod node;
pub(crate) mod nodes;
pub mod parser;
pub(crate) mod properties;
pub mod queries;
pub mod scope_gate;
pub(crate) mod transformer;
pub mod tree_sitter;

pub use self::{
    db::{Database, Db},
    node::Node,
    properties::Properties,
    scope_gate::{Node as ScopeNode, ScopeGate},
};

#[salsa::jar(db = crate::db::Db)]
pub struct Jar(
    crate::parser::FileSource,
    crate::parser::Diagnostics,
    crate::parser::parse,
    crate::parser::inner_transform,
    crate::parser::NodeSource,
    crate::queries::ClosestNodeQuery,
    crate::queries::find_namespace,
    crate::tree_sitter::FileSource,
    crate::tree_sitter::parse,
);
