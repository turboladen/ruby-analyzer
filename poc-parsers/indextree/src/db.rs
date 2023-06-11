use std::sync::{Arc, Mutex};

use salsa::DebugWithDb;

pub trait Db: salsa::DbWithJar<crate::Jar> {}
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<crate::Jar> {}

#[derive(Default)]
#[salsa::db(crate::Jar)]
pub struct Database {
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
