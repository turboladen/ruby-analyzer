use std::path::{Path, PathBuf};

use lib_ruby_parser::traverse::visitor::Visitor;
use ropey::Rope;

use crate::{location::LocNode, transformer, ScopedIndex};

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

/// Diagnostics (ex. errors) emitted during the parsing process. Can be retrieved via
/// `crate::parser::parse::accumulated::<Diagnostics>(db)`.
///
#[salsa::accumulator]
pub struct Diagnostics(lib_ruby_parser::Diagnostic);

/// This is the main entry point / purpose to this crate. Takes source code from a single file,
/// parses it using lib-ruby-parser, then transforms the lib-ruby-parser output to our custom
/// `Node`s.
///
#[salsa::tracked]
pub fn parse(db: &dyn crate::db::Db, file_source: FileSource) -> (Vec<LocNode>, ScopedIndex) {
    let file_uri = file_source.file_uri(db);
    let code = file_source.code(db);

    let result = lrp_parse(file_uri, code);

    for diagnostic in result.diagnostics {
        Diagnostics::push(db, diagnostic);
    }

    if let Some(root_node) = result.ast {
        let node_source = NodeSource::new(db, *root_node);
        inner_transform(db, node_source)
    } else {
        Default::default()
    }
}

/// Just a wrapper for calling `lib_ruby_parser`'s parse function.
///
pub(crate) fn lrp_parse(buffer_name: &Path, code: &Rope) -> lib_ruby_parser::ParserResult {
    let options = lib_ruby_parser::ParserOptions {
        buffer_name: buffer_name.to_string_lossy().to_string(),
        decoder: None,
        token_rewriter: None,
        record_tokens: false,
    };

    let parser = lib_ruby_parser::Parser::new(code.to_string(), options);

    parser.do_parse()
}

#[salsa::input]
pub(crate) struct NodeSource {
    #[return_ref]
    pub(crate) root_node: lib_ruby_parser::Node,
}

/// Uses a `Transformer` to take the AST result of a `lib_ruby_parser::ParserResult` and converts
/// those `Node`s to our `Node`s.
///
#[salsa::tracked]
pub(crate) fn inner_transform(
    db: &dyn crate::db::Db,
    node_source: NodeSource,
) -> (Vec<LocNode>, ScopedIndex) {
    let root_node = node_source.root_node(db);

    let mut transformer = transformer::Transformer::default();
    transformer.visit(root_node);

    transformer.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn parse_valid_ruby_test() {
        let db = crate::db::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; end");

        let file_source = FileSource::new(&db, file_uri, code);

        let (loc_nodes, index) = parse(&db, file_source);
        assert_eq!(2, loc_nodes.len());
        assert_eq!(2, index.len());

        let (loc_nodes, index) = parse(&db, file_source);
        assert_eq!(2, loc_nodes.len());
        assert_eq!(2, index.len());
    }

    #[test]
    fn parse_invalid_ruby_test() {
        let db = crate::db::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; ");

        let file_source = FileSource::new(&db, file_uri, code);

        let (loc_nodes, index) = parse(&db, file_source);
        assert!(loc_nodes.is_empty());
        assert_eq!(1, index.len());

        let diags = parse::accumulated::<Diagnostics>(&db, file_source);
        assert_eq!(diags.len(), 1);
    }
}
