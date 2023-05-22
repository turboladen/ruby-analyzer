use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use ropey::Rope;
use salsa::DebugWithDb;

use crate::{namespace::Namespace, parser::FileSource, Node};

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
    pub fn parse_source(&mut self, file_uri: PathBuf, code: Rope) {
        let db = self as &dyn crate::db::Db;

        let file_source = FileSource::new(db, file_uri.clone(), code);
        let nodes = crate::parser::parse(db, file_source);
        self.file_index.insert(file_uri, nodes);
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
