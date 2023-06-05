use crate::{
    location::{Contains, LocNode},
    scope_gate::ScopeGate,
};

#[salsa::input]
pub struct ClosestNodeQuery {
    pub offset: usize,

    #[return_ref]
    pub loc_nodes: Vec<LocNode>,
}

#[salsa::tracked]
pub fn find_scope_gate(db: &dyn crate::db::Db, query: ClosestNodeQuery) -> Option<ScopeGate> {
    let offset = query.offset(db);
    let loc_nodes = query.loc_nodes(db);

    loc_nodes
        .iter()
        .filter(|n| n.expression_l().contains(offset))
        // .inspect(|n| {
        //     dbg!(n);
        // })
        .map(|node| node.scope_gate().clone())
        .max_by(|x, y| x.len().cmp(&y.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod find_scope_gate {
        use std::path::PathBuf;

        use ropey::Rope;

        use crate::{
            db::{Database, Db},
            parser::FileSource,
            scope_gate,
            scoped_index::{self, nodes::*, NodeProperties},
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
            let (loc_nodes, scoped_index) = crate::parser::parse(&db, file_source);

            let expected_root = ScopeGate::default();

            let nodes_in_scope_gate = [
                scoped_index::Node {
                    id: 2,
                    properties: NodeProperties::Const(Const {
                        name: "Foo".to_string(),
                        scope_id: None,
                    }),
                },
                scoped_index::Node {
                    id: 1,
                    properties: NodeProperties::Class(Class {
                        name: "Foo".to_string(),
                        name_id: 2,
                        superclass_id: None,
                        body_id: None,
                    }),
                },
            ];

            // At the beginning of the class def.
            {
                let query = ClosestNodeQuery::new(&db, 0, loc_nodes.clone());
                let scope_gate = find_scope_gate(&db, query).unwrap();
                assert_eq!(&scope_gate, &expected_root);
                assert_eq!(scoped_index[&scope_gate], nodes_in_scope_gate);
            }

            // Right after the ;.
            {
                let query = ClosestNodeQuery::new(&db, 10, loc_nodes);
                let scope_gate = find_scope_gate(&db, query).unwrap();

                let expected_foo = expected_root.join(scope_gate::Node::Class("Foo".to_string()));
                assert_eq!(scope_gate, expected_foo);

                let expected_nodes = [scoped_index::Node {
                    id: 3,
                    properties: NodeProperties::EmptyBody,
                }];
                assert_eq!(scoped_index[&scope_gate], expected_nodes);
            }
        }

        #[tracing_test::traced_test]
        #[test]
        fn parse_valid_dual_level_test() {
            let db = Database::default();
            let file_source = file_source(&db, "class Foo; module Bar; end; end");
            let (loc_nodes, index) = crate::parser::parse(&db, file_source);

            let expected_foo = ScopeGate::new(vec![scope_gate::Node::Class("Foo".to_string())]);

            let nodes_in_root_gate = [
                scoped_index::Node {
                    id: 2,
                    properties: NodeProperties::Const(Const {
                        name: "Foo".to_string(),
                        scope_id: None,
                    }),
                },
                scoped_index::Node {
                    id: 1,
                    properties: NodeProperties::Class(Class {
                        name: "Foo".to_string(),
                        name_id: 2,
                        superclass_id: None,
                        body_id: Some(3),
                    }),
                },
            ];

            // At the beginning of the class def.
            {
                let query = ClosestNodeQuery::new(&db, 0, loc_nodes.clone());

                let scope_gate = find_scope_gate(&db, query).unwrap();
                assert_eq!(scope_gate, ScopeGate::default());
                assert_eq!(index[&scope_gate], nodes_in_root_gate);
            }

            // Right after the first ;.
            {
                let query = ClosestNodeQuery::new(&db, 10, loc_nodes.clone());
                let scope_gate = find_scope_gate(&db, query).unwrap();
                assert_eq!(scope_gate, ScopeGate::default());
                assert_eq!(index[&scope_gate], nodes_in_root_gate);
            }

            let nodes_in_foo_gate = [
                scoped_index::Node {
                    id: 4,
                    properties: NodeProperties::Const(Const {
                        name: "Bar".to_string(),
                        scope_id: None,
                    }),
                },
                scoped_index::Node {
                    id: 3,
                    properties: NodeProperties::Module(Module {
                        name: "Bar".to_string(),
                        name_id: 4,
                        body_id: None,
                    }),
                },
            ];

            // On the first "m" in "module"
            {
                let query = ClosestNodeQuery::new(&db, 11, loc_nodes.clone());
                let scope_gate = find_scope_gate(&db, query).unwrap();
                assert_eq!(scope_gate, expected_foo);
                assert_eq!(index[&scope_gate], nodes_in_foo_gate);
            }

            let expected_bar = ScopeGate::new(vec![
                scope_gate::Node::Class("Foo".to_string()),
                scope_gate::Node::Module("Bar".to_string()),
            ]);

            // In between `Bar;` and `end`
            {
                let nodes_in_bar_gate = [scoped_index::Node {
                    id: 5,
                    properties: NodeProperties::EmptyBody,
                }];
                let query = ClosestNodeQuery::new(&db, 22, loc_nodes.clone());
                let scope_gate = find_scope_gate(&db, query).unwrap();
                assert_eq!(scope_gate, expected_bar);
                assert_eq!(index[&scope_gate], nodes_in_bar_gate);
            }

            // After "Bar"'s "end"'s ";"
            {
                let query = ClosestNodeQuery::new(&db, 27, loc_nodes);
                let scope_gate = find_scope_gate(&db, query).unwrap();
                assert_eq!(scope_gate, expected_foo);
                assert_eq!(index[&scope_gate], nodes_in_root_gate);
            }
        }
    } /* find_scope_gate */
}
