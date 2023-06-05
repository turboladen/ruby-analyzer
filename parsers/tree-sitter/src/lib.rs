pub mod db;
pub mod parser;

#[salsa::jar(db = crate::db::Db)]
pub struct Jar(
    crate::parser::FileSource,
    crate::parser::Diagnostics,
    crate::parser::parse,
);
