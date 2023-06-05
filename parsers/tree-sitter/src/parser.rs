use std::path::PathBuf;

use ropey::Rope;
use tree_sitter::{Parser, Tree};

#[salsa::input]
pub struct FileSource {
    #[id]
    #[return_ref]
    pub file_uri: PathBuf,

    #[return_ref]
    pub code: Rope,

    pub tree: Option<Tree>,
}

/// Diagnostics (ex. errors) emitted during the parsing process. Can be retrieved via
/// `crate::parser::parse::accumulated::<Diagnostics>(db)`.
///
#[salsa::accumulator]
pub struct Diagnostics(lib_ruby_parser::Diagnostic);

#[salsa::tracked(no_eq)]
pub fn parse(db: &dyn crate::db::Db, file_source: FileSource) -> Option<Tree> {
    let mut parser = Parser::new();
    let code = file_source.code(db);
    let tree = file_source.tree(db);

    parser
        .set_language(tree_sitter_ruby::language())
        .expect("Error loading Ruby grammar");

    parser.parse(code.bytes().collect::<Vec<u8>>(), tree.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Bummer that we don't get any errors/diagnostics from TS.
    //
    #[test]
    fn experiment_with_invalid_code() {
        let db = crate::db::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; ");

        let file_source = FileSource::new(&db, file_uri, code.clone(), None);
        let tree = parse(&db, file_source).unwrap();
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

        let query = tree_sitter::Query::new(
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
]"#,
        )
        .unwrap();

        let mut query_cursor = tree_sitter::QueryCursor::new();
        let c = code.to_string();
        let matches = query_cursor.matches(&query, tree.root_node(), c.as_bytes());

        for m in matches {
            if m.captures.is_empty() {
                continue;
            }
            dbg!(&m);
            for capture in m.captures {
                dbg!(capture.node.id());
                dbg!(capture.node.kind());
                dbg!(capture.node.kind_id());
                dbg!(capture.node.range());
                dbg!(capture.node.to_sexp());
            }
        }

        // panic!("show me the things");
    }

    // Bummer that we don't get any errors/diagnostics from TS.
    //
    #[test]
    fn parse_invalid_code() {
        let db = crate::db::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; ");

        let file_source = FileSource::new(&db, file_uri, code, None);
        let tree = parse(&db, file_source).unwrap();

        let mut program = tree.walk();
        assert_eq!(program.node().child_count(), 1);
        assert_eq!(program.node().named_child_count(), 1);

        assert!(program.goto_first_child());
        assert_eq!(program.node().kind(), "class");
        assert!(program.node().has_error());
        assert_eq!(program.node().child_count(), 4);
        assert_eq!(program.node().named_child_count(), 1);

        assert!(program.goto_first_child());
        assert!(program.goto_next_sibling());
        assert!(program.goto_next_sibling());
        assert!(program.goto_next_sibling());
        assert!(program.node().has_error());
    }

    #[test]
    fn parse_valid_ruby_test() {
        let db = crate::db::Database::default();
        let file_uri = PathBuf::from("/tmp/test.rb");
        let code = Rope::from_str("class Foo; end");

        let file_source = FileSource::new(&db, file_uri, code, None);
        let tree = parse(&db, file_source).unwrap();

        let mut program = tree.walk();
        assert_eq!(program.node().child_count(), 1);
        assert_eq!(program.node().named_child_count(), 1);

        assert!(program.goto_first_child());
        assert_eq!(program.node().kind(), "class");
        assert!(!program.node().has_error());
        assert_eq!(program.node().child_count(), 4);
        assert_eq!(program.node().named_child_count(), 1);

        assert!(program.goto_first_child());
        assert!(program.goto_next_sibling());
        assert!(program.goto_next_sibling());
        assert!(program.goto_next_sibling());
        assert!(!program.node().has_error());
    }
}
