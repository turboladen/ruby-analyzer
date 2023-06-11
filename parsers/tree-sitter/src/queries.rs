use ropey::Rope;
use tree_sitter::{Node, Point, Query, QueryCursor};

/// Given a byte offset in the code, this finds the child-most (most childly? childish?) node.
/// Essentially, this is useful for finding the most relevant node for a cursor's position in
/// an IDE.
///
#[must_use]
pub fn node_at_byte(offset: usize, root_node: Node<'_>) -> Option<Node<'_>> {
    let mut cursor = root_node.walk();
    let mut deepest_node = None;

    while cursor.goto_first_child_for_byte(offset).is_some() {
        let node = cursor.node();
        let _ = deepest_node.insert(node);
    }

    deepest_node
}

/// Just like `node_at_byte()`, but this takes a `Point` (0-based row/column) instead.
///
#[must_use]
pub fn node_at_point(point: Point, root_node: Node<'_>) -> Option<Node<'_>> {
    let mut cursor = root_node.walk();
    let mut deepest_node = None;

    while cursor.goto_first_child_for_point(point).is_some() {
        let node = cursor.node();
        let _ = deepest_node.insert(node);
    }

    deepest_node
}

/// Gets all scope gates in the given code. Scope gates consist of:
///
/// - classes
/// - modules
/// - singleton methods (aka class methods)
/// - methods (aka instance methods)
/// - blocks (both `{ |var| ... }` and `do |var|; end`)
///
#[must_use]
pub fn scope_gates<'a>(root_node: Node<'a>, code: &'a Rope) -> Vec<Node<'a>> {
    const SCOPES: &str = "
(class) @scope
(module) @scope

(method) @scope
(singleton_method) @scope

[
 (block)
 (do_block) 
 ] @scope";

    let query = Query::new(tree_sitter_ruby::language(), SCOPES).unwrap();
    let mut query_cursor = QueryCursor::new();

    let all_matches = query_cursor.matches(&query, root_node, |node: Node<'_>| {
        code.byte_slice(node.byte_range())
            .chunks()
            .map(str::as_bytes)
    });
    let raise_idx = query.capture_index_for_name("scope").unwrap();

    all_matches
        .flat_map(|each_match| {
            each_match
                .captures
                .iter()
                .filter(|c| c.index == raise_idx)
                .map(|capture| capture.node)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    use super::*;

    mod node_at_byte {
        use super::*;

        #[tracing_test::traced_test]
        #[test]
        fn offset_at_leaf_test() {
            let code = Rope::from_str(
                "
class Foo
  def bar
    @stuff = 42
  end
end",
            );

            let result = parse(&code, None).unwrap();
            // Byte 35 is the `2` in `42`
            let node = node_at_byte(35, result.tree().root_node()).unwrap();
            assert_eq!(node.kind(), "integer");
        }
    }

    mod node_at_point {
        use super::*;

        #[tracing_test::traced_test]
        #[test]
        fn offset_at_leaf_test() {
            let code = Rope::from_str(
                "
class Foo
  def bar
    @stuff = 42
  end
end",
            );

            let result = parse(&code, None).unwrap();
            // Row 3, column 7 is the `u` in `@stuff`
            let node = node_at_point(Point::new(3, 7), result.tree().root_node()).unwrap();
            dbg!(&node);
            assert_eq!(node.kind(), "instance_variable");
        }
    }

    mod scope_gates {
        use super::*;

        #[test]
        fn classes_test() {
            let code = Rope::from_str(
                "
class Foo
  module Foo
    def self.bobo
      do_something_else { |foo| puts foo }
      nil
    end

    def meow(var)
      puts var

      some_thing do |tmp|
         puts tmp * 100 
      end

      42
    end
  end
end",
            );

            let parse_result = parse(&code, None).unwrap();
            let matches = scope_gates(parse_result.tree().root_node(), &code);
            dbg!(&matches);

            assert_eq!(matches.len(), 6);

            assert_eq!(matches[0].kind(), "class");
            assert_eq!(matches[1].kind(), "module");
            assert_eq!(matches[2].kind(), "singleton_method");
            assert_eq!(matches[3].kind(), "block");
            assert_eq!(matches[4].kind(), "method");
            assert_eq!(matches[5].kind(), "do_block");
        }
    } /* scope_gates */
}
