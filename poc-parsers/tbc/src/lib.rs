pub mod db;
pub mod location;
pub(crate) mod lrp_extensions;
pub mod parser;
pub mod queries;
pub mod scope_gate;
pub mod scoped_index;
pub(crate) mod transformer;

pub use self::{
    db::{Database, Db},
    scope_gate::{Node as ScopeGateNode, ScopeGate},
    scoped_index::ScopedIndex,
};

#[salsa::jar(db = crate::db::Db)]
pub struct Jar(
    crate::parser::FileSource,
    crate::parser::Diagnostics,
    crate::parser::parse,
    crate::parser::inner_transform,
    crate::parser::NodeSource,
    crate::queries::ClosestNodeQuery,
    crate::queries::find_scope_gate,
);
