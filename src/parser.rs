use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use lib_ruby_parser::traverse::visitor::Visitor;
use ropey::Rope;

use crate::{node::Node, transformer};

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
pub fn parse(db: &dyn crate::db::Db, file_source: FileSource) -> Arc<Vec<Node>> {
    let file_uri = file_source.file_uri(db);
    let code = file_source.code(db);

    let result = lrp_parse(file_uri, code);

    for diagnostic in result.diagnostics {
        Diagnostics::push(db, diagnostic);
    }

    if let Some(root_node) = result.ast {
        let node_source = NodeSource::new(db, *root_node);
        Arc::new(inner_transform(db, node_source))
    } else {
        Arc::new(Vec::with_capacity(0))
    }
}

pub fn parse_ts(
    db: &dyn crate::db::Db,
    file_source: FileSource,
) -> std::option::Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    let code = file_source.code(db);

    parser
        .set_language(tree_sitter_ruby::language())
        .expect("Error loading Ruby grammar");

    parser.parse(code.bytes().collect::<Vec<u8>>(), None)
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
pub(crate) fn inner_transform(db: &dyn crate::db::Db, node_source: NodeSource) -> Vec<Node> {
    let root_node = node_source.root_node(db);

    let mut transformer = transformer::Transformer::new();
    transformer.visit(root_node);

    transformer.into_nodes()
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

        let nodes = parse(&db, file_source);
        assert_eq!(2, nodes.len());

        let nodes = parse(&db, file_source);
        assert_eq!(2, nodes.len());
    }

    #[test]
    fn parse_invalid_ruby_test() {
        let db = crate::db::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; ");

        let file_source = FileSource::new(&db, file_uri, code);

        let nodes = parse(&db, file_source);
        assert!(nodes.is_empty());

        let diags = parse::accumulated::<Diagnostics>(&db, file_source);
        assert_eq!(diags.len(), 1);
    }

    // Bummer that we don't get any errors/diagnostics from TS.
    //
    #[test]
    fn ts_parse_invalid_code() {
        let db = crate::db::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; ");

        let file_source = FileSource::new(&db, file_uri, code.clone());
        let tree = parse_ts(&db, file_source).unwrap();
        dbg!(&tree);

        let mut program = tree.walk();
        assert_eq!(program.node().child_count(), 1);
        assert_eq!(program.node().named_child_count(), 1);

        assert!(program.goto_first_child());
        dbg!(program.node());
        assert_eq!(program.node().kind(), "class");
        assert!(program.node().has_error());
        assert_eq!(program.node().child_count(), 4);
        assert_eq!(program.node().named_child_count(), 1);

        assert!(program.goto_first_child());
        dbg!(program.node());
        assert!(!program.node().has_error());

        assert!(program.goto_next_sibling());
        dbg!(program.node());
        assert!(!program.node().has_error());

        assert!(program.goto_next_sibling());
        dbg!(program.node());
        assert!(!program.node().has_error());

        assert!(program.goto_next_sibling());
        dbg!(program.node());
        assert!(program.node().has_error());

        //  (#eq? @name "Foo")
        let query =
            // tree_sitter::Query::new(tree_sitter_ruby::language(), "(program (class))").unwrap();
            tree_sitter::Query::new(
                tree_sitter_ruby::language(),
                r#"[
  (class
    name: [
      (constant) @name
      (scope_resolution
        name: (#eq? @name "Foo"))
    ]) @definition.class
  (singleton_class
    value: [
      (constant) @name
      (scope_resolution
        name: (#eq? @name "Foo"))
    ]) @definition.class
]"#
            ).unwrap();

        let mut query_cursor = tree_sitter::QueryCursor::new();
        let c = code.to_string();
        let matches = query_cursor.matches(&query, tree.root_node(), c.as_bytes());

        for m in matches {
            if m.captures.is_empty() {
                continue;
            }
            dbg!(&m);
            for capture in m.captures {
                dbg!(capture.node);
            }
        }

        panic!("hinm");
    }

    // #[test]
    // fn ts_parse_valid_ruby_test() {
    //     let db = crate::db::Database::default();
    //     let file_uri = PathBuf::from("/tmp/test.rb");
    //     let code = Rope::from_str("class Foo; end");

    //     let file_source = FileSource::new(&db, file_uri, code);

    //     let nodes = parse(&db, file_source);
    //     assert_eq!(2, nodes.len());

    //     let nodes = parse(&db, file_source);
    //     assert_eq!(2, nodes.len());
    // }
}
