pub(crate) mod compat;
pub(crate) mod db;
pub(crate) mod lrp_extensions;
pub(crate) mod namespace;
pub(crate) mod node;
pub(crate) mod nodes;
pub mod parser;
pub(crate) mod properties;
pub(crate) mod transformer;

#[salsa::jar(db = Db)]
pub struct Jar(
    crate::parser::FileAst,
    crate::parser::FileSource,
    crate::parser::Diagnostics,
    crate::parser::parse,
);

pub trait Db: salsa::DbWithJar<Jar> {}
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
