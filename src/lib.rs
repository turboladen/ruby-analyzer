use std::sync::{Arc, Mutex};

use salsa::DebugWithDb;

pub(crate) mod compat;
pub(crate) mod db;
pub(crate) mod lrp_extensions;
pub(crate) mod namespace;
pub(crate) mod node;
pub(crate) mod nodes;
pub mod parser;
pub(crate) mod properties;
pub(crate) mod transformer;

pub trait Db: salsa::DbWithJar<Jar> {}
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}

#[salsa::jar(db = Db)]
pub struct Jar(
    crate::parser::FileAst,
    crate::parser::FileSource,
    crate::parser::Diagnostics,
    crate::parser::parse,
);

#[derive(Default)]
#[salsa::db(crate::Jar)]
pub(crate) struct Database {
    storage: salsa::Storage<Self>,
    logs: Option<Arc<Mutex<Vec<String>>>>,
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
        })
    }
}
