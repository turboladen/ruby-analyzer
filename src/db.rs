use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use lib_ruby_parser::Diagnostic;
use ropey::Rope;
use salsa::DebugWithDb;

use crate::{
    namespace::{self, Namespace},
    parser::{Diagnostics, FileSource},
    properties::Properties,
    Node,
};

pub trait Db: salsa::DbWithJar<crate::Jar> {}
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<crate::Jar> {}

#[derive(Default)]
#[salsa::db(crate::Jar)]
pub struct Database {
    storage: salsa::Storage<Self>,
    logs: Option<Arc<Mutex<Vec<String>>>>,
    file_index: HashMap<PathBuf, Vec<Node>>,
}

impl Database {
    pub fn parse_source(&mut self, file_uri: PathBuf, code: Rope) -> Vec<Diagnostic> {
        let db = self as &dyn crate::db::Db;

        let file_source = FileSource::new(db, file_uri.clone(), code);
        let nodes = crate::parser::parse(db, file_source);
        let diags = crate::parser::parse::accumulated::<Diagnostics>(db, file_source);
        self.file_index.insert(file_uri, nodes);

        diags
    }

    /// Iterates through all Nodes for `file_uri` and finds the deepest-most `Namespace` for where
    /// the `offset` is.
    ///
    pub fn find_namespace(&self, file_uri: &Path, offset: usize) -> Option<Namespace> {
        let nodes = self.file_index.get(file_uri)?;

        nodes
            .iter()
            .filter(move |n| n.expression_l().begin <= offset && offset <= n.expression_l().end)
            .map(|node| match node.properties() {
                Properties::Class(cp) => node.namespace().join(namespace::Node::Class {
                    name: cp.name.clone(),
                }),
                Properties::Module(cp) => node.namespace().join(namespace::Node::Module {
                    name: cp.name.clone(),
                }),
                _ => node.namespace().clone(),
            })
            .max_by(|x, y| x.len().cmp(&y.len()))
    }
}

impl salsa::Database for Database {
    fn salsa_event(&self, event: salsa::Event) {
        // Log interesting events, if logging is enabled
        if let Some(logs) = &self.logs {
            // don't log boring events
            if let salsa::EventKind::WillExecute { .. } = event.kind {
                logs.lock()
                    .unwrap()
                    .push(format!("Event: {:?}", event.debug(self)));
            }
        }
    }
}

impl salsa::ParallelDatabase for Database {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Database {
            storage: self.storage.snapshot(),
            logs: self.logs.clone(),
            file_index: self.file_index.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod find_namespace {
        use super::*;

        #[test]
        fn parse_valid_single_level_test() {
            let mut db = Database::default();
            let file_uri = PathBuf::from("/tmp/test.rb");
            let code = Rope::from_str("class Foo; end");

            let diags = db.parse_source(file_uri.clone(), code);
            assert!(diags.is_empty(), "Diagnostics not empty! {:?}", diags);

            // At the beginning of the class def.
            let namespace = db.find_namespace(&file_uri, 0).unwrap();
            let expected = Namespace::new(vec![namespace::Node::Class {
                name: "Foo".to_string(),
            }]);
            assert_eq!(namespace, expected);

            // Right after the ;.
            let namespace = db.find_namespace(&file_uri, 10).unwrap();
            assert_eq!(namespace, expected);
        }

        #[test]
        fn parse_valid_dual_level_test() {
            let mut db = Database::default();
            let file_uri = PathBuf::from("/tmp/test.rb");
            let code = Rope::from_str("class Foo; module Bar; end; end");

            let diags = db.parse_source(file_uri.clone(), code);
            assert!(diags.is_empty(), "Diagnostics not empty! {:?}", diags);

            let expected_foo = Namespace::new(vec![namespace::Node::Class {
                name: "Foo".to_string(),
            }]);

            // At the beginning of the class def.
            let namespace = db.find_namespace(&file_uri, 0).unwrap();
            assert_eq!(namespace, expected_foo);

            // Right after the first ;.
            let namespace = db.find_namespace(&file_uri, 10).unwrap();
            assert_eq!(namespace, expected_foo);

            let expected_bar = Namespace::new(vec![
                namespace::Node::Class {
                    name: "Foo".to_string(),
                },
                namespace::Node::Module {
                    name: "Bar".to_string(),
                },
            ]);

            // On the first "m" in "module"
            let namespace = db.find_namespace(&file_uri, 11).unwrap();
            assert_eq!(namespace, expected_bar);

            // After "Bar"'s "end"'s ";"
            let namespace = db.find_namespace(&file_uri, 27).unwrap();
            assert_eq!(namespace, expected_foo);
        }
    } /* find_namespace */
}
