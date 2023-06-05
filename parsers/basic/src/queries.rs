use std::sync::Arc;

use crate::{
    properties::Properties,
    scope_gate::{self, ScopeGate},
    Node,
};

#[salsa::input]
pub struct ClosestNodeQuery {
    pub offset: usize,

    #[return_ref]
    pub nodes: Arc<Vec<Node>>,
}

/// Iterates through all Nodes for `file_uri` and finds the deepest-most `Namespace` for where
/// the `offset` is.
///

#[salsa::tracked]
pub fn find_namespace(db: &dyn crate::db::Db, query: ClosestNodeQuery) -> Option<ScopeGate> {
    let offset = query.offset(db);
    let nodes = query.nodes(db);

    nodes
        .iter()
        .filter(move |n| n.expression_l().begin() <= offset && offset <= n.expression_l().end())
        .map(|node| match node.properties() {
            Properties::Class(cp) => node
                .scope_gate()
                .join(scope_gate::Node::Class(cp.name.clone())),
            Properties::Module(cp) => node
                .scope_gate()
                .join(scope_gate::Node::Module(cp.name.clone())),
            Properties::Def(cp) => node
                .scope_gate()
                .join(scope_gate::Node::Def(cp.name.clone())),
            Properties::Defs(cp) => node
                .scope_gate()
                .join(scope_gate::Node::Defs(cp.name.clone())),
            _ => node.scope_gate().clone(),
        })
        .max_by(|x, y| x.len().cmp(&y.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod find_namespace {
        use std::path::PathBuf;

        use ropey::Rope;

        use crate::{
            db::{Database, Db},
            parser::FileSource,
        };

        use super::*;

        fn file_source(db: &dyn Db, code: &str) -> FileSource {
            let file_uri = PathBuf::from("/tmp/test.rb");
            let code = Rope::from_str(code);

            FileSource::new(db, file_uri, code)
        }

        #[test]
        fn parse_valid_single_level_test() {
            let db = Database::default();
            let file_source = file_source(&db, "class Foo; end");
            let nodes = crate::parser::parse(&db, file_source);

            let expected = ScopeGate::new(vec![scope_gate::Node::Class("Foo".to_string())]);

            // At the beginning of the class def.
            {
                let query = ClosestNodeQuery::new(&db, 0, nodes.clone());
                let namespace = find_namespace(&db, query).unwrap();
                assert_eq!(namespace, expected);
            }

            // Right after the ;.
            {
                let query = ClosestNodeQuery::new(&db, 10, nodes);
                let namespace = find_namespace(&db, query).unwrap();
                assert_eq!(namespace, expected);
            }
        }

        #[test]
        fn parse_valid_dual_level_test() {
            let db = Database::default();
            let file_source = file_source(&db, "class Foo; module Bar; end; end");
            let nodes = crate::parser::parse(&db, file_source);

            let expected_foo = ScopeGate::new(vec![scope_gate::Node::Class("Foo".to_string())]);

            // At the beginning of the class def.
            {
                let query = ClosestNodeQuery::new(&db, 0, nodes.clone());

                let namespace = find_namespace(&db, query).unwrap();
                assert_eq!(namespace, expected_foo);
            }

            // Right after the first ;.
            {
                let query = ClosestNodeQuery::new(&db, 10, nodes.clone());
                let namespace = find_namespace(&db, query).unwrap();
                assert_eq!(namespace, expected_foo);
            }

            let expected_bar = ScopeGate::new(vec![
                scope_gate::Node::Class("Foo".to_string()),
                scope_gate::Node::Module("Bar".to_string()),
            ]);

            // On the first "m" in "module"
            {
                let query = ClosestNodeQuery::new(&db, 11, nodes.clone());
                let namespace = find_namespace(&db, query).unwrap();
                assert_eq!(namespace, expected_bar);

                // After "Bar"'s "end"'s ";"
                let query = ClosestNodeQuery::new(&db, 27, nodes);
                let namespace = find_namespace(&db, query).unwrap();
                assert_eq!(namespace, expected_foo);
            }
        }
    } /* find_namespace */
}
