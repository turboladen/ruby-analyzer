use ropey::Rope;
use tree_sitter::{Parser, Tree};
use tree_sitter_ruby::language;

use crate::ParseResult;

// pub use self::{diagnostic::Diagnostic, output::Output};

#[must_use]
pub fn parse<'a>(code: &'a Rope, tree: Option<&Tree>) -> Option<ParseResult<'a>> {
    let mut parser = Parser::new();

    parser
        .set_language(language())
        .expect("Error loading Ruby grammar");

    let tree = parser.parse(code.bytes().collect::<Vec<u8>>(), tree)?;

    Some(ParseResult::new(tree, code))
}

#[cfg(test)]
mod tests {
    use tracing::debug;
    use tree_sitter::{Query, QueryCursor};

    use crate::diagnostic::DiagType;

    use super::*;

    #[test]
    fn experiment_with_invalid_code() {
        let code = Rope::from_str("class Foo; ");
        let output = parse(&code, None).unwrap();

        dbg!(&output);

        let mut program = output.tree().walk();
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

        let query = Query::new(
            language(),
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

        let mut query_cursor = QueryCursor::new();
        let c = output.code().to_string();
        let matches = query_cursor.matches(&query, output.tree().root_node(), c.as_bytes());

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
        let code = Rope::from_str("class Foo; ");
        let output = parse(&code, None).unwrap();

        let mut program = output.tree().walk();
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
        let code = Rope::from_str("class Foo; end");
        let output = parse(&code, None).unwrap();

        let mut program = output.tree().walk();
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

    #[test]
    #[should_panic(expected = "Queries don't handle MISSING nodes")]
    fn experiment_with_invalid_code2() {
        let code = Rope::from_str("class Foo; ");
        let output = parse(&code, None).unwrap();

        let program = output.tree().walk();
        dbg!(program.node().to_sexp());

        // TODO: This fails because querying for `MISSING` nodes isn't yet supported.
        // https://github.com/tree-sitter/tree-sitter/issues/606
        // https://github.com/tree-sitter/tree-sitter/issues/650
        let query = Query::new(language(), r#"(MISSING) @missing"#)
            .expect("Queries don't handle MISSING nodes");

        let mut query_cursor = QueryCursor::new();
        let c = output.code().to_string();
        let all_matches = query_cursor.matches(&query, output.tree().root_node(), c.as_bytes());

        // get the index of the capture named "raise"
        let raise_idx = query.capture_index_for_name("missing").unwrap();

        for each_match in all_matches {
            // iterate over all captures called "raise"
            // ignore captures such as "fn-name"
            for capture in each_match.captures.iter().filter(|c| c.index == raise_idx) {
                let range = capture.node.range();
                let text = &c[range.start_byte..range.end_byte];
                let line = range.start_point.row;
                let col = range.start_point.column;
                debug!("[Line: {line}, Col: {col}] Offending source code: `{text}`",);
            }
        }
    }

    #[test]
    fn diagnostics_missing_test() {
        let code = Rope::from_str("class Foo; ");
        let output = parse(&code, None).unwrap();
        let diags = output.diagnostics();

        dbg!(&diags);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].diag_type(), DiagType::Missing);
    }

    #[test]
    fn diagnostics_error_test() {
        let code = Rope::from_str(
            r#"class Foo
        '
        end"#,
        );
        let output = parse(&code, None).unwrap();

        let diags = output.diagnostics();

        dbg!(&diags);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].diag_type(), DiagType::Error);
    }
}
