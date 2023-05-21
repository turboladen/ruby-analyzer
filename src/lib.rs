pub(crate) mod compat;
pub(crate) mod db;
pub(crate) mod file_ast;
pub(crate) mod lrp_extensions;
pub(crate) mod namespace;
pub(crate) mod node;
pub(crate) mod nodes;
pub mod parser;
pub(crate) mod properties;
pub(crate) mod transformer;

use std::path::PathBuf;

use ropey::Rope;

use crate::parser::NodeSource;

pub use self::{file_ast::FileAst, node::Node};

#[salsa::jar(db = Db)]
pub struct Jar(
    crate::file_ast::FileAst,
    FileSource,
    Diagnostics,
    parse,
    crate::parser::inner_transform,
    crate::parser::NodeSource,
);

pub trait Db: salsa::DbWithJar<Jar> {}
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}

/// The path and contents of a source file. Typically, this is what we parse.
///
#[salsa::input]
pub struct FileSource {
    #[id]
    #[return_ref]
    pub file_uri: PathBuf,

    #[return_ref]
    pub code: Rope,
}

#[salsa::accumulator]
pub struct Diagnostics(lib_ruby_parser::Diagnostic);

#[salsa::tracked]
pub fn parse(db: &dyn crate::Db, file_source: FileSource) -> FileAst {
    let file_uri = file_source.file_uri(db);
    let code = file_source.code(db);

    let result = self::parser::lrp_parse(file_uri, code);

    let nodes = if let Some(root_node) = result.ast {
        let node_source = NodeSource::new(db, *root_node, code.clone());
        self::parser::inner_transform(db, node_source)
    } else {
        vec![]
    };

    for diagnostic in result.diagnostics {
        Diagnostics::push(db, diagnostic);
    }

    FileAst::new(db, file_uri.clone(), nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_ruby_test() {
        let db = crate::db::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; end");

        let file_source = FileSource::new(&db, file_uri, code);

        let ast = parse(&db, file_source);
        let nodes = ast.nodes(&db);

        assert_eq!(2, nodes.len());

        let ast = parse(&db, file_source);
        let nodes = ast.nodes(&db);

        assert_eq!(2, nodes.len());
    }

    #[test]
    fn parse_invalid_ruby_test() {
        let db = crate::db::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; ");

        let file_source = FileSource::new(&db, file_uri, code);

        let ast = parse(&db, file_source);
        let nodes = ast.nodes(&db);
        assert!(nodes.is_empty());

        let diags = parse::accumulated::<Diagnostics>(&db, file_source);
        assert_eq!(diags.len(), 1);
    }
}
